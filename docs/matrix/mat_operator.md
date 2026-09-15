# ktm mat operator

## operator[]

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| i | `size_t` | 列下标，不检查越界 |
| 返回值 | `vec<Col, T>&` | 第 i 列的引用 |

## operator*（矩阵 × 向量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<Row, Col, T>&` | 任意规模矩阵 |
| v | `const vec<Row, T>&` | 元素个数等于 m 列数的向量 |
| 返回值 | `vec<Col, T>` | 列向量的线性组合 `Σv[i]·m[i]` |

## operator*（向量 × 矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<Col, T>&` | 元素个数等于 m 行数的向量 |
| m | `const mat<Row, Col, T>&` | 任意规模矩阵 |
| 返回值 | `vec<Row, T>` | 行向量左乘矩阵 |

## operator*（矩阵 × 矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m1 | `const mat<Row, Col, T>&` | 左矩阵 |
| m2 | `const mat<U, Row, T>&` | 右矩阵（列数任意，行数须等于 m1 列数） |
| 返回值 | `mat<U, Col, T>` | 矩阵乘积 |

## operator*=

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m2 | `const mat<Row, Row, T>&` | 同规模方阵 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，`m1 = m1 * m2`（仅方阵可用） |

## operator+（矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `mat<Row, Col, T>` | 逐分量相加 |

## operator-（矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `mat<Row, Col, T>` | 逐分量相减 |

## operator+=（矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量加赋值 |

## operator-=（矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量减赋值 |

## operator-（一元）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 输入矩阵 |
| 返回值 | `mat<Row, Col, T>` | 逐分量取反 |

## operator+（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 矩阵 |
| s | `T` | 标量（`s + x` 形式亦可用） |
| 返回值 | `mat<Row, Col, T>` | 逐分量加标量 |

## operator-（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 矩阵 |
| s | `T` | 标量（无 `s − x` 形式） |
| 返回值 | `mat<Row, Col, T>` | 逐分量减标量 |

## operator+=（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量加赋值 |

## operator-=（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量减赋值 |

## operator*（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 矩阵 |
| s | `T` | 标量（`s * x` 形式亦可用） |
| 返回值 | `mat<Row, Col, T>` | 逐分量乘标量 |

## operator/（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 矩阵 |
| s | `T` | 标量（无 `s / x` 形式） |
| 返回值 | `mat<Row, Col, T>` | 逐分量除标量 |

## operator*=（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量乘赋值 |

## operator/=（标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| s | `T` | 标量 |
| 返回值 | `mat<Row, Col, T>&` | 自身引用，逐分量除赋值 |

## ktm_op_madd（矩阵 × 标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 被加矩阵 |
| y | `const mat<Row, Col, T>&` | 同型矩阵（乘数） |
| s | `T` | 被乘标量 |
| 返回值 | `mat<Row, Col, T>` | 新矩阵 `x + y*s`（浮点基类型使用 fma） |

## ktm_op_smadd（矩阵 × 标量）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `mat<Row, Col, T>&` | 被加矩阵 |
| y | `const mat<Row, Col, T>&` | 同型矩阵（乘数） |
| s | `T` | 被乘标量 |
| 返回值 | `mat<Row, Col, T>&` | 就地 `x += y*s`，返回 x 引用（浮点基类型使用 fma） |

## ktm_op_madd（标量 × 矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 被加矩阵 |
| s | `T` | 乘数标量 |
| z | `const mat<Row, Col, T>&` | 同型矩阵（被乘数） |
| 返回值 | `mat<Row, Col, T>` | 新矩阵 `x + s*z`（浮点基类型使用 fma） |

## ktm_op_smadd（标量 × 矩阵）

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `mat<Row, Col, T>&` | 被加矩阵 |
| s | `T` | 乘数标量 |
| z | `const mat<Row, Col, T>&` | 同型矩阵（被乘数） |
| 返回值 | `mat<Row, Col, T>&` | 就地 `x += s*z`，返回 x 引用（浮点基类型使用 fma） |

## operator==

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序相等判断 |

## operator!=

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序不等判断 |

## operator<

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序小于判断 |

## operator>

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序大于判断 |

## operator<=

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序小于等于判断 |

## operator>=

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| x | `const mat<Row, Col, T>&` | 左矩阵 |
| y | `const mat<Row, Col, T>&` | 同型右矩阵 |
| 返回值 | `bool` | 按列向量的字典序大于等于判断 |

## operator<<

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| out | `std::ostream&` | 输出流（支持 char 与 wchar_t 两套流） |
| m | `const mat<Row, Col, T>&` | 输出矩阵 |
| 返回值 | 流引用 | 按列顺序空格分隔输出全部元素 |

## operator>>

T 支持类型：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| in | `std::istream&` | 输入流（支持 char 与 wchar_t 两套流） |
| m | `mat<Row, Col, T>&` | 输入矩阵 |
| 返回值 | 流引用 | 按列顺序依次读入 |
