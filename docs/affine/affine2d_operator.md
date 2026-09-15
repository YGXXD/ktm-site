# ktm affine2d operator

## operator<<

T 支持类型：float / double

**重载 1（仿射变换）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| affine | `const affine2d&` | 右复合的仿射变换 |
| 返回值 | `affine2d&` | 自身引用，等价 concat |

**重载 2（2×2 矩阵）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| matrix | `const mat<2, 2, T>&` | 右复合的线性矩阵 |
| 返回值 | `affine2d&` | 自身引用，等价 concat |

**重载 3（3×3 矩阵）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| matrix | `const mat<3, 3, T>&` | 右复合的齐次矩阵 |
| 返回值 | `affine2d&` | 自身引用，等价 concat |

## operator>>

T 支持类型：float / double

**重载 1（2×2 矩阵）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| out_matrix | `mat<2, 2, T>&` | 输出矩阵引用 |
| 返回值 | `affine2d&` | 自身引用，等价 matrix2x2 |

**重载 2（3×3 矩阵）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| out_matrix | `mat<3, 3, T>&` | 输出矩阵引用 |
| 返回值 | `affine2d&` | 自身引用，等价 matrix3x3 |

**重载 3（4×4 矩阵）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| out_matrix | `mat<4, 4, T>&` | 输出矩阵引用 |
| 返回值 | `affine2d&` | 自身引用，等价 matrix4x4 |
