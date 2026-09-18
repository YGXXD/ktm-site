# ktm guide: 最佳实践与性能

## 命名空间：避免与 std 冲突

ktm 的 `min` / `max` / `clamp` / `less` / `abs` 等与 `<algorithm>`、`<cmath>` 中的同名函数存在冲突。若同时 `using namespace ktm;` 与 `using namespace std;`，调用会产生二义性：

```c++
using namespace ktm;
using namespace std;

min(1.f, 2.f);      // 编译错误：ktm::min 与 std::min 二义
```

推荐做法（按优先级）：

1. **不引入整个命名空间**，按需限定：`ktm::fvec3 v;`；
2. 只引入 `using namespace ktm;`，std 侧用 `std::` 前缀；
3. 若两者都已引入，冲突函数用 `ktm::` 显式限定。

```c++
ktm::min(1.f, 2.f);      // 显式指定 ktm 版本
ktm::clamp(x, lo, hi);
```

> ktm 版本的 `min` / `max` 在浮点时使用 `fmin` / `fmax` 语义（对 NaN 的处理与 std 版本不同），且额外提供向量重载，这是推荐使用 ktm 版本的原因。

## 精度选择：float 还是 double

- 图形渲染（顶点变换、MVP）：`float` 足够，SIMD 加速收益最大（`fvec` / `fmat` / `fquat`）；
- 物理模拟、累计运算、科学计算：建议 `double`（`dvec` / `dmat` / `dquat`）；
- 避免在类型间来回转换：`fvec3 → dvec3 → fvec3` 每次都是逐分量静态转换，有精度损失与转换开销。

## fast:: 命名空间：以精度换性能

对性能极敏感的场景（粒子系统、每帧海量归一化），ktm 提供 `fast::` 低精度版本：

```c++
fast::sqrt(x)      // 快速平方根
fast::rsqrt(x)     // 快速平方根倒数
fast::recip(x)     // 快速倒数

fast::length(v)        // 向量模长
fast::normalize(v)     // 向量归一化
fast::distance(a, b)   // 距离
fast::project(a, b)    // 投影
```

`fast::` 仅支持 `float` / `double`，精度低于标准版本，视觉类计算通常可接受，物理关键路径慎用。

## 融合乘加：ktm_op_madd

`x + y * z` 形式的计算请用 `ktm_op_madd` / `ktm_op_smadd`，浮点基类型自动映射到 fma 指令（一次完成、单次舍入）：

```c++
float r = ktm_op_madd(1.f, 2.f, 3.f);   // 1 + 2*3 = 7

fvec3 x, y, z;
fvec3 r = ktm_op_madd(x, y, z);   // 新向量 x + y*z
ktm_op_smadd(x, y, z);            // 就地 x += y*z，返回引用可继续链式
```

支持向量×向量、向量×标量、标量×向量、矩阵×标量四种组合。这两个函数定义在全局命名空间（`<ktm/ktm_op.h>`），不依赖 ktm 其他头文件。

## SIMD 说明

- ktm 在编译期自动选择平台最优指令集（x86 SSE/AVX、ARM NEON、WASM SIMD）；
- 无需任何配置；交叉编译时按目标平台选择编译参数即可；
- `vec` / `mat` / `quat` / `comp` 的核心运算（算术、乘法、归约）都有 SIMD 实现；
- 4 维及以下的 `fvec` / `fmat` / `fquat` 是 SIMD 收益最大的尺寸，渲染热点代码优先使用它们。

## 浮点比较：永远用 compare 函数

`operator==` 对浮点几乎总是错误的选择：

```c++
0.1f + 0.2f == 0.3f;              // false！
ktm::equal(0.1f + 0.2f, 0.3f, 1e-5f);   // true

// 向量版本：各分量绝对值之和 ≤ 容差
ktm::equal(v1, v2, 1e-4f);

// 严格序比较同样带容差
ktm::less(a, b);      // a - b < -e
ktm::greater(a, b);   // a - b > e
```

容差默认 `epsilon<T>`，可按业务尺度显式传入。

## 常见陷阱清单

| 陷阱 | 说明 |
|:-|:-|
| 角度单位 | ktm 全部使用**弧度**，转换用 `radians()` / `degrees()` |
| 矩阵下标 | `mat<Row, Col>` 中 `Row` 是**列数**；`m[i]` 取第 i **列** |
| 四元数布局 | 存储顺序 `(i, j, k, r)`，**实部在最后** |
| 复数布局 | 存储顺序 `(i, r)`，虚部在前 |
| 单位向量 | `angle_axis`、`rotate3d_axis`、`normalize` 的轴 / 输入须为单位或非零向量 |
| 归一化零向量 | `normalize` 对零输入未定义（几何版），四元数 / 复数版安全返回单位元 |
| 向量乘法 | `v1 * v2` 是**逐分量**乘，点积是 `dot(v1, v2)` |
| 标量运算符 | `标量 - 向量`、`标量 / 向量` 形式不存在 |
| swizzle 模版方法 | `swizzle<Ns...>()` 仅 N > 4 的通用向量提供，vec2/3/4 用具名混洗（`xy()`、`zyx()` 等） |
| 随机数 | 基于 `std::rand()`，需要可复现结果时用 `std::srand()` 播种 |

## 随机数工具

```c++
lerp_rand(0.f, 1.f)        // [min, max) 均匀分布
gauss_rand(0.f, 1.f)      // 正态分布 N(mean, deviation²)（Box-Muller）
exp_rand(1.f)             // 指数分布，密度 λ·e^(−λx)

circur_rand(1.f)          // 圆周上均匀点
disk_rand(1.f)            // 圆盘内面积均匀点
sphere_rand(1.f)          // 球面上均匀点
ball_rand(1.f)            // 球体内体积均匀点
```

后四个是图形学采样常用分布：`disk_rand` / `ball_rand` 已对半径做 sqrt / cbrt 修正，保证面积 / 体积均匀，无需自己再处理。

## 头文件纪律

`<ktm/ktm.h>` 包含全部类型与函数，编译开销较大。大型项目中建议：

- 通用公共头只包含实际用到的模块头（如 `<ktm/type_vec.h>` + `<ktm/function/geometric.h>`）；
- `.cpp` 内部再包含 `<ktm/ktm.h>` 无妨，避免把它放进被广泛包含的公共头。

## API 参考

- [common 函数](/docs/function?item=common)（含 fast:: 全表）
- [compare 函数](/docs/function?item=compare)
- [random 函数](/docs/function?item=random)
