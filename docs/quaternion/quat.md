# ktm quat

## 类型定义

| 类型 | 定义 | 解释 |
|:-|:-|:-|
| `quat<T>` | 模版类 | `T` 为浮点类型 |
| `fquat` | `quat<float>` | 单精度四元数 |
| `dquat` | `quat<double>` | 双精度四元数 |

## 数据成员

T 支持类型：float / double

| 成员 | 类型 | 解释 |
|:-|:-|:-|
| i / j / k | `T` | 虚部分量（存储顺序在前） |
| r | `T` | 实部分量（存储顺序在后） |

## 构造函数

T 支持类型：float / double

**重载 1（默认）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| （无参） | — | 全部分量初始化为 0 |

**重载 2（分量构造）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x, y, z, w | `T...` | 依次赋给 i、j、k、r（实部在最后） |

**重载 3（向量构造）**

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<4, T>&` | v.x→i、v.y→j、v.z→k、v.w→r |

## real

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `T` | 实部 r |

## imag

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `vec<3, T>` | 虚部向量 (i, j, k) |

## angle

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `T` | 旋转角 `2·atan2(|imag|, real)` |

## axis

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `vec<3, T>` | 归一化旋转轴 `normalize(imag)` |

## matrix3x3

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `mat<3, 3, T>` | 等价旋转矩阵 |

## matrix4x4

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `mat<4, 4, T>` | 齐次旋转矩阵（[3][3] = 1，其余边界为 0） |

## to_array

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `std::array<T, 4>&` | 按 (i, j, k, r) 顺序转换为数组引用 |

## data

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | 指针 | 底层数据 |

## begin

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | 迭代器 | 指向首个分量 i 的正向迭代器 |

## end

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | 迭代器 | 正向遍历末尾 |

## rbegin

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | 反向迭代器 | 指向末分量 r，反向遍历起点 |

## rend

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | 反向迭代器 | 反向遍历末尾 |

## cbegin

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | const 正向迭代器 | 只读遍历起点 |

## cend

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | const 正向迭代器 | 只读遍历末尾 |

## crbegin

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | const 反向迭代器 | 只读反向遍历起点 |

## crend

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | const 反向迭代器 | 只读反向遍历末尾 |

## size

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `size_t` | 恒为 4 |

## max_size

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `size_t` | 恒为 4 |

## empty

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `bool` | 恒为 false |

## at

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| i | `size_t` | 分量下标，带越界检查并抛出异常 |
| 返回值 | `T&` | 分量引用 |

## front

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `T&` | 首个分量 i 的引用 |

## back

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `T&` | 末个分量 r 的引用 |

## to_string

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `std::string` | 空格分隔的 i j k r 字符串 |

## to_wstring

T 支持类型：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `std::wstring` | 空格分隔的宽字符 i j k r 字符串 |
