# ktm api: quaternion 函数

## conjugate

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `quat<T>` | 共轭 `(−i, −j, −k, r)`，表示反向旋转 |

## inverse

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `quat<T>` | 逆四元数 `conj(q)/|q|²`；单位四元数时等价于共轭 |

## lerp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| p | `const quat<T>&` | 起始四元数 |
| q | `const quat<T>&` | 结束四元数 |
| t | `T` | 插值系数 |
| 返回值 | `quat<T>` | 逐分量线性插值，结果非单位四元数，使用前须归一化 |

## dot

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| p | `const quat<T>&` | 四元数 |
| q | `const quat<T>&` | 四元数 |
| 返回值 | `T` | 四分量点积，衡量两旋转的接近程度 |

## length

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `T` | 模长 |

## normalize

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `quat<T>` | 单位四元数；零输入返回 (0, 0, 0, 1) |

## exp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `quat<T>` | 四元数指数 `e^r·(cos|v|, sin|v|·v/|v|)`，v 为虚部 |

## log

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| q | `const quat<T>&` | 四元数 |
| 返回值 | `quat<T>` | 四元数对数 `(ln|q|, θ·axis)` |

## slerp

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 起始四元数 |
| y | `const quat<T>&` | 结束四元数 |
| t | `T` | 插值系数 |
| 返回值 | `quat<T>` | 沿最短弧的球面插值，角速度恒定 |

## slerp_longest

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 起始四元数 |
| y | `const quat<T>&` | 结束四元数 |
| t | `T` | 插值系数 |
| 返回值 | `quat<T>` | 沿最长弧的球面插值，用于超过 180° 的旋转动画 |

## slerp_internal

**T 支持类型**：float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const quat<T>&` | 起始四元数 |
| y | `const quat<T>&` | 结束四元数 |
| t | `T` | 插值系数 |
| 返回值 | `quat<T>` | 无路径修正的原始球面插值 |
