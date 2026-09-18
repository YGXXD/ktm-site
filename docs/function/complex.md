# ktm api: complex 函数

## conjugate

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `comp<T>` | 共轭 `(−i, r)`，表示反向旋转 |

## inverse

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `comp<T>` | 逆复数 `conj(c)/|c|²`；单位复数时等价于共轭 |

## lerp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const comp<T>&` | 起始复数 |
| y | `const comp<T>&` | 结束复数 |
| t | `T` | 插值系数 |
| 返回值 | `comp<T>` | 逐分量线性插值，结果非单位复数，使用前须归一化 |

## dot

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const comp<T>&` | 复数 |
| y | `const comp<T>&` | 复数 |
| 返回值 | `T` | 二分量点积 |

## length

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `T` | 模长 `√(r² + i²)` |

## normalize

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `comp<T>` | 单位复数；零输入返回 (0, 1) |

## exp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `comp<T>` | 复数指数 `e^r·(sin(i), cos(i))`，即欧拉公式 `e^(r+iθ) = e^r(cosθ + isinθ)` |

## log

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| c | `const comp<T>&` | 复数 |
| 返回值 | `comp<T>` | 复数对数 `(θ, ln|c|)`，θ 为辐角 |

## slerp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const comp<T>&` | 起始复数 |
| y | `const comp<T>&` | 结束复数 |
| t | `T` | 插值系数 |
| 返回值 | `comp<T>` | 按 `from_to(y, x)` 有向角（范围 (−π, π]）的球面插值 |

## slerp_longest

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const comp<T>&` | 起始复数 |
| y | `const comp<T>&` | 结束复数 |
| t | `T` | 插值系数 |
| 返回值 | `comp<T>` | 沿最长弧的球面插值（补角方向） |

## slerp_internal

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const comp<T>&` | 起始复数 |
| y | `const comp<T>&` | 结束复数 |
| t | `T` | 插值系数 |
| 返回值 | `comp<T>` | 按 [0, 2π) 正向角的原始球面插值 |
