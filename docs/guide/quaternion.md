# ktm guide: 四元数

四元数是表达三维旋转的最佳数据结构：无万向节锁、存储小、插值自然。ktm 的 `quat<T>` 要求 `T` 为浮点类型（`fquat` / `dquat`）。

## 内存布局

`quat` 按虚部在前、实部在后的顺序存储 4 个分量：

| 成员 | 含义 |
|:-|:-|
| `i` / `j` / `k` | 虚部分量（对应旋转轴 × sin(θ/2)） |
| `r` | 实部分量（cos(θ/2)） |

构造时参数顺序同样遵循 `(i, j, k, r)`——**实部在最后**：

```c++
fquat q(0.f, 0.f, 0.f, 1.f);          // 单位四元数
fquat v(fvec4(0.f, 0.f, 0.f, 1.f));   // 由 vec4 构造：x→i、y→j、z→k、w→r
```

## 构造旋转

全部通过静态方法完成：

```c++
// 绕任意单位轴旋转 angle 弧度
fquat q1 = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));

// 绕坐标轴旋转
fquat qx = fquat::from_angle_x(0.5f);
fquat qy = fquat::from_angle_y(0.5f);
fquat qz = fquat::from_angle_z(0.5f);

// 单位四元数（无旋转）
fquat id = fquat::identity();

// 由实部与虚部向量构造
fquat ri = fquat::real_imag(1.f, fvec3(0.f, 0.f, 0.f));

// 把 from 方向旋转到 to 方向（自动处理 180° 与接近相反的情况）
fquat ft = fquat::from_to(fvec3(0.f, 0.f, 1.f), fvec3(0.f, 1.f, 0.f));

// 由旋转矩阵构造（3×3 或 4×4，4×4 取左上 3×3）
fquat fm = fquat::from_matrix(fmat3x3::from_eye());

// 观察朝向：以 +direction（lh）或 −direction（rh）为前向
fquat look = fquat::look_to_lh(fvec3(0.f, 0.f, 1.f), fvec3(0.f, 1.f, 0.f));
```

## 应用旋转

```c++
fvec3 v { 5.f, -5.f, 1.f };
fvec3 rotated = q1 * v;         // 四元数直接旋转向量

// 复合旋转：q1 先作用，再作用 q2，等价于 q2 * q1 * v
fquat combined = q2 * q1;
q1 *= q2;                        // 就地复合
```

> 旋转一个向量用 `q * v`；旋转的复合用四元数乘法，注意顺序与矩阵一致（右边的先作用）。

## 读取旋转信息

```c++
q1.real()          // 实部 r
q1.imag()          // 虚部向量 (i, j, k)
q1.angle()         // 旋转角 2·atan2(|imag|, real)
q1.axis()          // 归一化旋转轴 normalize(imag)
q1.matrix3x3()     // 等价 3×3 旋转矩阵
q1.matrix4x4()     // 齐次 4×4 旋转矩阵
```

四元数与矩阵互转验证：

```c++
fvec3 v { 5.f, -5.f, 1.f };
fquat q = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));
// 两种方式结果完全一致
std::cout << q * v << std::endl;
std::cout << q.matrix3x3() * v << std::endl;
```

## 四元数函数

位于 `ktm::` 命名空间（头文件 `<ktm/function/quaternion.h>`）：

```c++
conjugate(q)      // 共轭 (−i, −j, −k, r)，表示反向旋转
inverse(q)        // 逆 q̄/|q|²；单位四元数时等价于共轭
normalize(q)      // 归一化；零输入安全返回 (0, 0, 0, 1)
dot(p, q)         // 四分量点积，衡量两旋转接近程度
length(q)         // 模长
lerp(p, q, t)     // 逐分量线性插值（结果非单位，用前须 normalize）
slerp(p, q, t)    // 沿最短弧的球面插值（推荐）
slerp_longest(p, q, t)  // 沿最长弧插值，用于超过 180° 的旋转动画
slerp_internal(p, q, t) // 无路径修正的原始球面插值
exp(q) / log(q)   // 四元数指数 / 对数
```

## 插值实践

```c++
fquat a = fquat::from_angle_x(0.f);
fquat b = fquat::from_angle_x(half_pi<float>);

// 平滑旋转动画：每帧推进 t
fquat frame = slerp(a, b, 0.25f);   // 走最短弧，角速度恒定
```

`slerp` 会自动取两四元数间夹角小于 180° 的路径（必要时翻转 `y` 的符号）；若刻意要走"绕远路"（例如累计旋转超过 180° 的动画），使用 `slerp_longest`。

## 注意事项

- **单位四元数**才表示纯旋转，复合多次运算后建议重新 `normalize`；
- `angle_axis` 的轴必须是**单位向量**，角度一律使用**弧度**；
- `operator==` 等比较运算符是四分量字典序比较，浮点旋转的相等判断请用 `ktm::equal`（见 [compare](/docs/function?item=compare)）；
- 存储顺序与 GLSL 的 `vec4(x, y, z, w)` 一致，`q.to_array()` 可直接上传 shader。

## API 参考

- [quat 类型](/docs/quaternion?item=quat_type)
- [quat 静态方法](/docs/quaternion?item=quat_static)
- [quat 运算符](/docs/quaternion?item=quat_operator)
- [quaternion 函数](/docs/function?item=quaternion)
