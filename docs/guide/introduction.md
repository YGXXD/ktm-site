# ktm guide: 认识 ktm

> **k**u**t**ori **m**ath，珂朵莉数学库，属于珂学家的图形数学库。

ktm 是一个面向图形学与游戏开发的 C++ 数学库，提供了向量、矩阵、四元数、复数、仿射变换五大基本类型，以及一套覆盖通用数学、几何、比较、随机、矩阵变换与矩阵分解的函数库。

## 核心特性

- **head-only**：无需编译库本身，包含头文件 `<ktm/ktm.h>` 即可接入项目；
- **API 层次清晰**：基本数学类由模版继承组件化实现静态 ECS，接口按命名空间与组件划分，查阅方便；
- **高性能跨平台**：SIMD 指令集加速（x86 / ARM / WASM），提供计算性能优化；
- **现代 C++**：要求 C++17 及以上，大量使用 `constexpr`、变参模版与类型萃取，编译期即可完成大量检查。

## 模块总览

引入 `<ktm/ktm.h>` 后，所有能力都位于 `ktm` 命名空间下，按模块划分如下：

| 模块 | 内容 | API 参考 |
|:-|:-|:-|
| 类型（type） | `vec` 向量、`mat` 矩阵、`quat` 四元数、`comp` 复数、`affine2d` / `affine3d` 仿射变换 | [vector](/docs/vector?item=vec_type)、[matrix](/docs/matrix?item=mat_type)、[quaternion](/docs/quaternion?item=quat_type)、[complex](/docs/complex?item=comp_type)、[affine](/docs/affine?item=affine3d_type) |
| 通用数学（common） | `abs`、`min` / `max`、`clamp`、`lerp`、三角函数、指数对数、`fast::` 快速低精度版本等 | [common](/docs/function?item=common) |
| 几何（geometric） | `dot`、`cross`、`length`、`normalize`、`reflect`、`refract`、投影等 | [geometric](/docs/function?item=geometric) |
| 比较（compare） | 带容差的 `equal` / `less` / `greater` 系列，浮点安全比较 | [compare](/docs/function?item=compare) |
| 随机（random） | 均匀、正态、指数分布，以及圆周 / 圆盘 / 球面 / 球体随机点 | [random](/docs/function?item=random) |
| 矩阵函数（matrix） | 代数（转置、行列式、逆）、2D / 3D 变换矩阵、相机与投影矩阵、LU / QR / SVD / 特征分解 | [matrix](/docs/function?item=matrix) |
| 四元数函数（quaternion） | `conjugate`、`inverse`、`normalize`、`slerp` 球面插值等 | [quaternion](/docs/function?item=quaternion) |
| 复数函数（complex） | `conjugate`、`inverse`、`normalize`、`slerp` 等 | [complex](/docs/function?item=complex) |

## 常用类型别名

ktm 为常用模版实参提供了简短别名，全部遵循同一命名规则：

| 前缀 | 分量类型 | 示例 |
|:-|:-|:-|
| `f` | `float` | `fvec3`、`fmat4x4`、`fquat`、`fcomp`、`faffine3d` |
| `d` | `double` | `dvec3`、`dmat4x4`、`dquat`、`dcomp`、`daffine3d` |
| `s` | `int` | `svec3`、`smat4x4`（仅向量与矩阵提供） |
| `u` | `unsigned int` | `uvec3`、`umat4x4`（仅向量与矩阵提供） |

## 常量

`ktm/type/basic.h` 提供了一组按类型模版化的数学常量：

| 常量 | 含义 |
|:-|:-|
| `zero<T>` / `one<T>` | 0 与 1 |
| `epsilon<T>` | 机器精度（`std::numeric_limits<T>::epsilon()`） |
| `euler<T>` | 自然常数 e |
| `pi<T>` | 圆周率 π |
| `two_pi<T>` | 2π |
| `half_pi<T>` | π/2 |
| `recip_pi<T>` | 1/π |
| `sqrt_two<T>` / `rsqrt_two<T>` | √2 与 1/√2 |

```c++
#include <ktm/ktm.h>
#include <iostream>

int main()
{
    using namespace ktm;

    std::cout << pi<float> << std::endl;       // 3.14159
    std::cout << half_pi<double> << std::endl;  // 1.5708
    std::cout << epsilon<float> << std::endl;  // 1.19209e-07
}
```

## 接下来

- [安装与接入](/docs/guide?item=installation)：把 ktm 加进你的项目；
- [快速上手](/docs/guide?item=quickstart)：十分钟跑通第一个示例；
- 之后按 [向量](/docs/guide?item=vector) → [矩阵](/docs/guide?item=matrix) → [四元数](/docs/guide?item=quaternion) → [复数](/docs/guide?item=complex) → [仿射变换](/docs/guide?item=affine) 的顺序阅读即可覆盖日常用法。
