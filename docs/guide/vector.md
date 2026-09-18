# ktm guide: 向量

向量是 ktm 中最基础也最常用的类型。`vec<N, T>` 支持任意维度 `N > 1` 与任意算术类型 `T`。

## 类型与别名

```c++
ktm::vec<3, float> a;   // 显式写法
ktm::fvec3 b;           // 常用别名
ktm::dvec3 c;           // double 分量
ktm::svec3 d;           // int 分量
ktm::uvec3 e;           // unsigned int 分量
ktm::vec<5, float> f;   // 高维向量同样可用
```

## 构造方式

```c++
fvec3 a;                          // (0, 0, 0) 全部分量为 0
fvec3 b(2.f);                     // (2, 2, 2) 标量填充
fvec3 c(1.f, 2.f, 3.f);           // 分量列举
fvec4 d(fvec3(1.f, 2.f, 3.f), 4.f);     // 低维拼接：vec3 + 标量 → vec4
fvec2 e = fvec4(1.f, 2.f, 3.f, 4.f).xy();  // 高维取低维：经具名混洗
dvec3 g = fvec3(1.5f, 2.5f, 3.5f);      // 分量类型转换：float → double
```

## 分量访问

```c++
fvec3 v { 1.f, 2.f, 3.f };

// 具名分量（vec2 / vec3 / vec4 提供，颜色命名 r/g/b/a 与 x/y/z/w 等价）
float x = v.x;   // 1
float g_ = v.g;   // 2（g 即 y）

// 下标访问（任意维度可用，不检查越界）
v[0] = 10.f;         // 通过 operator[]
v.at(1) = 20.f;       // at 带越界检查，越界抛出异常

// 底层数组
float* p = v.data();
std::array<float, 3>& arr = v.to_array();
```

## Swizzle 分量重排

**具名混洗**（vec2 / vec3 / vec4 提供 x/y/z/w 与 r/g/b/a 两套命名的全排列）：

```c++
fvec4 v { 1.f, 2.f, 3.f, 4.f };
fvec3 a = v.xyz();    // (1, 2, 3)
fvec3 b = v.wzyx();   // (4, 3, 2, 1) → vec4
fvec2 c = v.xy();     // (1, 2)
fvec3 d = v.bgr();    // (3, 2, 1) 颜色命名
```

**编译期下标混洗** `swizzle<Ns...>()`（面向 N > 4 的通用向量，vec2/3/4 请用具名混洗）：

```c++
vec<5, float> v { 1.f, 2.f, 3.f, 4.f, 5.f };
auto s = v.swizzle<2, 0>();   // (3, 1)，vec<2, float>
```

## 运算

```c++
fvec3 a { 1.f, 2.f, 3.f }, b { 4.f, 5.f, 6.f };

a + b          // 逐分量加
a - b          // 逐分量减
a * b          // 逐分量乘（注意：不是点积！）
a / b          // 逐分量除
a * 2.f        // 标量乘，2.f * a 亦可
a + 1.f        // 标量加，1.f + a 亦可
-a             // 逐分量取反
a += b;        // 复合赋值：+= -= *= /= 均支持（向量与标量两套）
```

> `operator-`（标量）与 `operator/`（标量）没有 `标量 - 向量`、`标量 / 向量` 形式。
> 向量之间的 `*` 是逐分量乘（Hadamard 积），点积请用 `dot` 函数。

**融合乘加** `ktm_op_madd` / `ktm_op_smadd`（浮点基类型使用 fma 指令，一次完成 `x + y*z`）：

```c++
fvec3 x { 1.f, 1.f, 1.f }, y { 2.f, 2.f, 2.f }, z { 3.f, 3.f, 3.f };
fvec3 r = ktm_op_madd(x, y, z);   // r = x + y * z，返回新向量
ktm_op_smadd(x, y, z);            // x += y * z，就地计算
```

## 比较

`== != < > <= >=` 按字典序逐分量比较（与 `std::array` 语义一致）。浮点向量的"相等"判断请使用带容差的 [compare 函数](/docs/function?item=compare)：

```c++
equal(a, b, 1e-5f)   // |a - b| 各分量绝对值之和 ≤ 容差
```

## 常用几何函数

```c++
dot(a, b)              // 点积
cross(a, b)            // 三维叉积；二维重载返回 vec3，z 分量为有向面积
length(a)              // 模长
length_squared(a)      // 模长平方（比较距离大小时可避免开方）
distance(a, b)         // 两点距离
normalize(a)           // 归一化（输入须非零）
project(a, b)          // a 在 b 上的投影
reflect(l, n)           // 入射向量关于单位法线的反射
refract(l, n, eta)      // 折射，eta 为折射率比
```

完整清单见 [geometric API 参考](/docs/function?item=geometric)。

## 逐分量数学函数

common 模块的函数几乎都提供标量与向量两套重载，向量版本即逐分量计算：

```c++
abs(a)         floor(a)     ceil(a)      round(a)
min(a, b)      max(a, b)    clamp(a, lo, hi)
lerp(a, b, t)  mix(a, b, t) step(edge, a) smoothstep(e0, e1, a)
sqrt(a)        rsqrt(a)     recip(a)     pow(a, b)
radians(a)     degrees(a)   sin(a)       cos(a) ...
```

归约函数把向量压缩为标量：

```c++
reduce_add(a)   // 全部分量之和
reduce_min(a)   // 最小分量
reduce_max(a)   // 最大分量
```

## 迭代与字符串

向量支持完整的 `std::array` 风格接口与流 IO：

```c++
fvec3 v { 1.5f, 2.5f, 3.5f };

for (float c : v) { }                  // begin/end、rbegin/rend、cbegin/cend 等
std::cout << v << std::endl;           // "1.5 2.5 3.5"
std::string s = v.to_string();         // "1.500000 2.500000 3.500000"
std::wcout << v.to_wstring() << std::endl;
std::cin >> v;                         // 依次读入 3 个分量
```

## API 参考

- [vec 类型](/docs/vector?item=vec_type)
- [vec 运算符](/docs/vector?item=vec_operator)
- [common 函数](/docs/function?item=common)
- [geometric 函数](/docs/function?item=geometric)
