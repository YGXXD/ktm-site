# ktm api: quat 运算符

## operator[]

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| i | `size_t` | 分量下标，不检查越界 |
| 返回值 | `T&` | 分量引用 |

## operator*（解引用）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| 返回值 | `vec<4, T>&` | 一元 `*q`，按 (i, j, k, r) 顺序重解释为四维向量的引用 |

## operator*（四元数乘）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>` | 哈密顿积（旋转复合：先应用 y，再应用 x） |

## operator*=

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>&` | 自身引用，就地复合 |

## operator*（旋转向量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 单位四元数 |
| v | `const vec<3, T>&` | 待旋转向量 |
| 返回值 | `vec<3, T>` | 旋转后的向量 |

## operator+（四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>` | 逐分量相加 |

## operator-（四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>` | 逐分量相减 |

## operator+=（四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>&` | 自身引用，逐分量加赋值 |

## operator-=（四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `quat<T>&` | 自身引用，逐分量减赋值 |

## operator-（一元）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 输入四元数 |
| 返回值 | `quat<T>` | 逐分量取反 |

## operator+（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 四元数 |
| s | `T` | 标量（`s + x` 形式亦可用） |
| 返回值 | `quat<T>` | 逐分量加标量 |

## operator-（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 四元数 |
| s | `T` | 标量（无 `s − x` 形式） |
| 返回值 | `quat<T>` | 逐分量减标量 |

## operator+=（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `quat<T>&` | 自身引用，逐分量加赋值 |

## operator-=（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `quat<T>&` | 自身引用，逐分量减赋值 |

## operator*（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 四元数 |
| s | `T` | 标量（`s * x` 形式亦可用） |
| 返回值 | `quat<T>` | 逐分量乘标量 |

## operator/（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 四元数 |
| s | `T` | 标量（无 `s / x` 形式） |
| 返回值 | `quat<T>` | 逐分量除标量 |

## operator*=（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `quat<T>&` | 自身引用，逐分量乘赋值 |

## operator/=（标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `quat<T>&` | 自身引用，逐分量除赋值 |

## ktm_op_madd（四元数 × 标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 被加四元数 |
| y | `const quat<T>&` | 乘数四元数 |
| s | `T` | 被乘标量 |
| 返回值 | `quat<T>` | 新四元数 `x + y*s`（使用 fma） |

## ktm_op_smadd（四元数 × 标量）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `quat<T>&` | 被加四元数 |
| y | `const quat<T>&` | 乘数四元数 |
| s | `T` | 被乘标量 |
| 返回值 | `quat<T>&` | 就地 `x += y*s`，返回 x 引用（使用 fma） |

## ktm_op_madd（标量 × 四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 被加四元数 |
| s | `T` | 乘数标量 |
| z | `const quat<T>&` | 被乘四元数 |
| 返回值 | `quat<T>` | 新四元数 `x + s*z`（使用 fma） |

## ktm_op_smadd（标量 × 四元数）

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `quat<T>&` | 被加四元数 |
| s | `T` | 乘数标量 |
| z | `const quat<T>&` | 被乘四元数 |
| 返回值 | `quat<T>&` | 就地 `x += s*z`，返回 x 引用（使用 fma） |

## operator==

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序相等判断 |

## operator!=

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序不等判断 |

## operator<

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序小于判断 |

## operator>

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序大于判断 |

## operator<=

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序小于等于判断 |

## operator>=

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 左四元数 |
| y | `const quat<T>&` | 右四元数 |
| 返回值 | `bool` | 按 (i, j, k, r) 顺序的字典序大于等于判断 |

## operator<<

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| out | `std::ostream&` | 输出流（支持 char 与 wchar_t 两套流） |
| q | `const quat<T>&` | 输出四元数 |
| 返回值 | 流引用 | 空格分隔输出 i j k r |

## operator>>

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| in | `std::istream&` | 输入流（支持 char 与 wchar_t 两套流） |
| q | `quat<T>&` | 输入四元数 |
| 返回值 | 流引用 | 依次读入 4 个分量 |
