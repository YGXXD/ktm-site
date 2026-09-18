# ktm api: matrix 函数

## transpose

**T 支持类型**：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<Row, Col, T>&` | 任意规模矩阵 |
| 返回值 | `mat<Col, Row, T>` | 转置矩阵 |

## diagonal

**T 支持类型**：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `vec<N, T>` | 对角线向量 |

## trace

**T 支持类型**：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `T` | 对角线元素之和 |

## determinant

**T 支持类型**：任意算术类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `T` | 行列式 |

## inverse

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵，要求行列式非零 |
| 返回值 | `mat<N, N, T>` | 逆矩阵 |

## rotate2d

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| 返回值 | `mat<3, 3, T>` | 绕原点旋转矩阵 |

## rotate2d_point

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| point | `const vec<2, T>&` | 旋转中心 |
| 返回值 | `mat<3, 3, T>` | 绕指定点旋转矩阵 |

## rotate2d_from_to

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| from | `const vec<2, T>&` | 起始方向向量 |
| to | `const vec<2, T>&` | 目标方向向量 |
| 返回值 | `mat<3, 3, T>` | 方向旋转矩阵 |

## translate2d

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<2, T>&` | 平移向量 |
| 返回值 | `mat<3, 3, T>` | 平移矩阵 |

## scale2d

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<2, T>&` | 缩放向量 |
| 返回值 | `mat<3, 3, T>` | 缩放矩阵 |

## shear2d_x

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle_y | `T` | 剪切角（弧度） |
| 返回值 | `mat<3, 3, T>` | 剪切矩阵，`x' = x + tan(angle_y)·y` |

## shear2d_y

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle_x | `T` | 剪切角（弧度） |
| 返回值 | `mat<3, 3, T>` | 剪切矩阵，`y' = y + tan(angle_x)·x` |

## look_to_lh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| eye_pos | `const vec<3, T>&` | 相机位置 |
| direction | `const vec<3, T>&` | 前向方向（左手系取 +direction 为前向） |
| up | `const vec<3, T>&` | 上方向 |
| 返回值 | `mat<4, 4, T>` | 观察矩阵 |

## look_to_rh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| eye_pos | `const vec<3, T>&` | 相机位置 |
| direction | `const vec<3, T>&` | 前向方向（右手系取 −direction 为前向） |
| up | `const vec<3, T>&` | 上方向 |
| 返回值 | `mat<4, 4, T>` | 观察矩阵 |

## look_at_lh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| eye_pos | `const vec<3, T>&` | 相机位置 |
| focus_pos | `const vec<3, T>&` | 注视目标位置 |
| up | `const vec<3, T>&` | 上方向 |
| 返回值 | `mat<4, 4, T>` | 观察矩阵（内部取 `normalize(focus_pos − eye_pos)` 为方向，左手系） |

## look_at_rh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| eye_pos | `const vec<3, T>&` | 相机位置 |
| focus_pos | `const vec<3, T>&` | 注视目标位置 |
| up | `const vec<3, T>&` | 上方向 |
| 返回值 | `mat<4, 4, T>` | 观察矩阵（内部取 `normalize(focus_pos − eye_pos)` 为方向，右手系） |

## perspective_lh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| fov_radians | `T` | 垂直视场角（弧度） |
| aspect | `T` | 宽高比 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 透视投影矩阵（左手系，深度范围 [0, 1]） |

## perspective_rh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| fov_radians | `T` | 垂直视场角（弧度） |
| aspect | `T` | 宽高比 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 透视投影矩阵（右手系，深度范围 [−1, 1]） |

## ortho_lh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| left | `T` | 左边界 |
| right | `T` | 右边界 |
| top | `T` | 上边界 |
| bottom | `T` | 下边界 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 正交投影矩阵（左手系） |

## ortho_rh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| left | `T` | 左边界 |
| right | `T` | 右边界 |
| top | `T` | 上边界 |
| bottom | `T` | 下边界 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 正交投影矩阵（右手系） |

## frustum_lh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| left | `T` | 左边界 |
| right | `T` | 右边界 |
| top | `T` | 上边界 |
| bottom | `T` | 下边界 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 视景体投影矩阵（左手系） |

## frustum_rh

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| left | `T` | 左边界 |
| right | `T` | 右边界 |
| top | `T` | 上边界 |
| bottom | `T` | 下边界 |
| znear | `T` | 近裁剪面距离 |
| zfar | `T` | 远裁剪面距离 |
| 返回值 | `mat<4, 4, T>` | 视景体投影矩阵（右手系） |

## rotate3d_x

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| 返回值 | `mat<4, 4, T>` | 绕 X 轴旋转矩阵 |

## rotate3d_y

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| 返回值 | `mat<4, 4, T>` | 绕 Y 轴旋转矩阵 |

## rotate3d_z

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| 返回值 | `mat<4, 4, T>` | 绕 Z 轴旋转矩阵 |

## rotate3d_axis

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| axis | `const vec<3, T>&` | 旋转轴，须为单位向量 |
| 返回值 | `mat<4, 4, T>` | 绕过原点轴的旋转矩阵 |

## rotate3d_from_to

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| from | `const vec<3, T>&` | 起始方向向量 |
| to | `const vec<3, T>&` | 目标方向向量 |
| 返回值 | `mat<4, 4, T>` | 方向旋转矩阵 |

## rotate3d_any_axis

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle | `T` | 旋转角（弧度） |
| axis_start | `const vec<3, T>&` | 轴上一点 |
| axis | `const vec<3, T>&` | 轴方向 |
| 返回值 | `mat<4, 4, T>` | 绕任意直线旋转矩阵 |

## translate3d

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<3, T>&` | 平移向量 |
| 返回值 | `mat<4, 4, T>` | 平移矩阵 |

## scale3d

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| v | `const vec<3, T>&` | 缩放向量 |
| 返回值 | `mat<4, 4, T>` | 缩放矩阵 |

## shear3d_x

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle_y | `T` | 沿 Y 方向的剪切角（弧度） |
| angle_z | `T` | 沿 Z 方向的剪切角（弧度） |
| 返回值 | `mat<4, 4, T>` | 剪切矩阵，`y' = y + tan(angle_y)·x`、`z' = z + tan(angle_z)·x` |

## shear3d_y

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle_x | `T` | 沿 X 方向的剪切角（弧度） |
| angle_z | `T` | 沿 Z 方向的剪切角（弧度） |
| 返回值 | `mat<4, 4, T>` | 剪切矩阵，`x' = x + tan(angle_x)·y`、`z' = z + tan(angle_z)·y` |

## shear3d_z

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| angle_x | `T` | 沿 X 方向的剪切角（弧度） |
| angle_y | `T` | 沿 Y 方向的剪切角（弧度） |
| 返回值 | `mat<4, 4, T>` | 剪切矩阵，`x' = x + tan(angle_x)·z`、`y' = y + tan(angle_y)·z` |

## reduce_hessenberg

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `reduce_component<M>` | get_transform() 为变换矩阵，get_reduce() 为化简结果（上海森堡矩阵），满足 `m = transformᵀ · reduce · transform` |

## reduce_tridiagonal

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 对称方阵 |
| 返回值 | `reduce_component<M>` | get_transform() 为变换矩阵，get_reduce() 为化简结果（三对角矩阵），满足 `m = transformᵀ · reduce · transform` |

## decompose_lu_doolittle

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `lu_component<M>` | get_l() 为单位下三角、get_u() 为上三角，满足 `m = L·U` |

## decompose_lu_crout

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `lu_component<M>` | get_l() 为下三角、get_u() 为单位上三角，满足 `m = L·U` |

## decompose_lu_cholesky

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 正定方阵 |
| 返回值 | `lu_component<M>` | get_l() 与 get_u() 互为转置，满足 `m = L·U`，`L = Uᵀ` |

## decompose_qr_householder

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `qr_component<M>` | get_q() 为正交矩阵、get_r() 为上三角矩阵，满足 `m = Q·R`（Householder 变换） |

## decompose_qr_givens

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `qr_component<M>` | get_q() 为正交矩阵、get_r() 为上三角矩阵，满足 `m = Q·R`（Givens 旋转） |

## decompose_qr_schmitd

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `qr_component<M>` | get_q() 为正交矩阵、get_r() 为上三角矩阵，满足 `m = Q·R`（Gram-Schmidt 正交化） |

## decompose_qr_on_hessenberg

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 上海森堡矩阵 |
| 返回值 | `qr_component<M>` | get_q() 为正交矩阵、get_r() 为上三角矩阵，满足 `m = Q·R`（针对上海森堡矩阵的快速分解） |

## decompose_qr_on_tridiagonal

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 三对角矩阵 |
| 返回值 | `qr_component<M>` | get_q() 为正交矩阵、get_r() 为上三角矩阵，满足 `m = Q·R`（针对三对角矩阵的快速分解） |

## decompose_edv_shiftqr

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 对称方阵 |
| 返回值 | `edv_component<M>` | get_vector() 为特征向量矩阵（每列一个特征向量）、get_value() 为特征值向量，满足 `m = V·diag(λ)·Vᵀ`（带位移 QR 迭代） |

## decompose_edv_jacobi

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 对称方阵 |
| 返回值 | `edv_component<M>` | get_vector() 为特征向量矩阵（每列一个特征向量）、get_value() 为特征值向量，满足 `m = V·diag(λ)·Vᵀ`（Jacobi 迭代） |

## decompose_svd

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<Row, Col, T>&` | 任意规模矩阵 |
| 返回值 | `svd_component<M>` | get_u()（`mat<Col, Col, T>`）、get_s()（`vec<min(Row,Col), T>`）、get_vt()（`mat<Row, Row, T>`），满足 `m = U·diag(s)·Vᵀ` |

## decompose_affine

**T 支持类型**：浮点类型

| 参数名 | 类型 | 解释 |
|:-|:-|:-|
| m | `const mat<N, N, T>&` | 方阵 |
| 返回值 | `affine_component<M>` | get_translate()、get_rotate()、get_shear()、get_scale() 四个矩阵，满足 `m = translate·rotate·shear·scale` |
