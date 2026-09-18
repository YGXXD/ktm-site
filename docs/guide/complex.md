# ktm guide: 复数

复数是二维旋转的最简表达。ktm 的 `comp<T>` 要求 `T` 为浮点类型（`fcomp` / `dcomp`），可以理解为"二维世界的四元数"。

## 内存布局

与四元数一致，虚部在前、实部在后：

| 成员 | 含义 |
|:-|:-|
| `i` | 虚部（对应 sin(θ)） |
| `r` | 实部（对应 cos(θ)） |

```c++
fcomp c(1.f, 0.f);          // (i=1, r=0)：旋转 90°
fcomp v(fvec2(1.f, 0.f));   // 由 vec2 构造：x→i、y→r
```

## 构造旋转

```c++
fcomp id = fcomp::identity();            // 单位复数 (0, 1)
fcomp a  = fcomp::from_angle(half_pi<float>);  // (sin, cos)，旋转 90°
fcomp ri = fcomp::real_imag(0.f, 1.f);   // (i=imag, r=real)，注意参数顺序实部在前

// 把 from 方向旋转到 to 方向
fcomp ft = fcomp::from_to(fvec2(1.f, 0.f), fvec2(0.f, 1.f));

// 由旋转矩阵构造（2×2，或 3×3 取左上 2×2）
fcomp fm = fcomp::from_matrix(fmat2x2::from_eye());
```

## 应用旋转

```c++
fcomp c = fcomp::from_angle(half_pi<float>);
fvec2 v { 1.f, 0.f };

fvec2 rotated = c * v;   // (≈0, 1)：逆时针旋转 90°
```

复数乘法即旋转复合：`c2 * c1 * v` 表示先经 `c1` 再经 `c2` 旋转，支持 `*=`。

## 读取旋转信息

```c++
c.real()        // 实部 r
c.imag()        // 虚部 i
c.angle()       // 辐角 atan2(i, r)
c.matrix2x2()   // 等价 2×2 旋转矩阵
c.matrix3x3()   // 等价 3×3 旋转矩阵（绕 Z 轴）
```

## 复数函数

位于 `ktm::` 命名空间（头文件 `<ktm/function/complex.h>`）：

```c++
conjugate(c)     // 共轭 (−i, r)，反向旋转
inverse(c)       // 逆 conj(c)/|c|²；单位复数时等价于共轭
normalize(c)     // 归一化；零输入安全返回 (0, 1)
dot(x, y)        // 二分量点积
length(c)        // 模长
lerp(x, y, t)    // 线性插值（结果非单位，用前须 normalize）
slerp(x, y, t)         // 按有向角（范围 (−π, π]）的球面插值
slerp_longest(x, y, t) // 沿最长弧（补角方向）插值
slerp_internal(x, y, t)// 按 [0, 2π) 正向角的原始插值
exp(c) / log(c)  // 复数指数（欧拉公式）/ 对数
```

## 与 affine2d 配合

复数与二维仿射变换天然配合，`affine2d::rotate` 直接接受复数或角度：

```c++
faffine2d affine {};
affine.translate(1.f, 2.f).rotate(fcomp::from_angle(half_pi<float>)).scale(2.f, 3.f);
```

## API 参考

- [comp 类型](/docs/complex?item=comp_type)
- [comp 静态方法](/docs/complex?item=comp_static)
- [comp 运算符](/docs/complex?item=comp_operator)
- [complex 函数](/docs/function?item=complex)
