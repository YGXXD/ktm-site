# ktm api: geometric 函数

## dot

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 任意维度向量 |
| y | `const vec<N, T>&` | 同型向量 |
| 返回值 | `T` | 点积 `Σxᵢyᵢ` |

## project

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 被投影向量 |
| y | `const vec<N, T>&` | 投影目标向量，非零 |
| 返回值 | `vec<N, T>` | x 在 y 上的投影向量 `dot(x,y)/dot(y,y)·y` |

## cross

**T 支持类型**：浮点类型

#### 重载 1（三维）

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<3, T>&` | 三维向量 |
| y | `const vec<3, T>&` | 同型向量 |
| 返回值 | `vec<3, T>` | 叉积向量，垂直于 x 与 y |

#### 重载 2（二维）

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<2, T>&` | 二维向量 |
| y | `const vec<2, T>&` | 同型向量 |
| 返回值 | `vec<3, T>` | `(0, 0, x.x·y.y − x.y·y.x)`，Z 分量为有向面积（正 = 逆时针） |

## length

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 任意维度向量 |
| 返回值 | `T` | 模长 `√dot(x,x)` |

## length_squared

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 任意维度向量 |
| 返回值 | `T` | 模长平方 `dot(x,x)`，比较大小时可避免开方 |

## distance

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 点 x |
| y | `const vec<N, T>&` | 同型点 y |
| 返回值 | `T` | 两点距离 `length(x−y)` |

## normalize

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 输入向量，非零 |
| 返回值 | `vec<N, T>` | 同方向单位向量 `rsqrt(dot(x,x))·x` |

## reflect

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 入射向量 |
| n | `const vec<N, T>&` | 同型单位法线 |
| 返回值 | `vec<N, T>` | 反射向量 `x − 2·dot(x,n)·n` |

## refract

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 入射单位向量 |
| n | `const vec<N, T>&` | 同型单位法线 |
| eta | `T` | 折射率比 |
| 返回值 | `vec<N, T>` | 折射向量；全反射时返回零向量 |

## fast::project

**T 支持类型**：仅支持 float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 被投影向量 |
| y | `const vec<N, T>&` | 投影目标向量，非零 |
| 返回值 | `vec<N, T>` | 同 project，除法使用 `fast::recip`，以精度换性能 |

## fast::length

**T 支持类型**：仅支持 float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 输入向量 |
| 返回值 | `T` | 同 length，开方使用 `fast::sqrt`，以精度换性能 |

## fast::distance

**T 支持类型**：仅支持 float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 点 x |
| y | `const vec<N, T>&` | 同型点 y |
| 返回值 | `T` | 同 distance，开方使用 `fast::sqrt`，以精度换性能 |

## fast::normalize

**T 支持类型**：仅支持 float / double

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const vec<N, T>&` | 输入向量，非零 |
| 返回值 | `vec<N, T>` | 同 normalize，平方根倒数使用 `fast::rsqrt`，以精度换性能 |
