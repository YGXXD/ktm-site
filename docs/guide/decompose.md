# ktm guide: 矩阵分解

`<ktm/function/matrix.h>` 提供了完整的矩阵分解工具链：化简、LU、QR、特征分解、SVD 与仿射分解。所有分解结果都封装为 `*_component` 结构体，通过 `get_xxx()` 访问器取用。

## 化简：上海森堡与三对角

把方阵经 Householder 变换化简为特殊形式，是后续快速分解的预处理步骤：

```c++
fmat3x3 m { { 4.f, 1.f, 0.f }, { 1.f, 3.f, -1.f }, { 0.f, -1.f, 2.f } };

auto hess = reduce_hessenberg(m);      // 任意方阵 → 上海森堡矩阵
hess.get_transform();                   // 变换矩阵 P
hess.get_reduce();                      // 化简结果 H，满足 m = Pᵀ · H · P

auto trid = reduce_tridiagonal(m);      // 对称方阵 → 三对角矩阵
trid.get_transform();
trid.get_reduce();
```

## LU 分解：解线性方程组

三种策略，均满足 `m = L·U`：

| 函数 | L | U | 适用 |
|:-|:-|:-|:-|
| `decompose_lu_doolittle` | 单位下三角 | 上三角 | 通用 |
| `decompose_lu_crout` | 下三角 | 单位上三角 | 通用 |
| `decompose_lu_cholesky` | 下三角 | 上三角（`U = Lᵀ`） | **正定**方阵，最快 |

```c++
fmat3x3 m { { 1.f, 2.f, 3.f }, { 4.f, 5.f, 6.f }, { 7.f, 8.f, 10.f } };

auto lu = decompose_lu_doolittle(m);
fmat3x3 check = lu.get_l() * lu.get_u();   // 还原 m

// 解方程 m·x = b：先解 L·y = b（前代），再解 U·x = y（回代）
```

## QR 分解：正交化

满足 `m = Q·R`，`Q` 正交、`R` 上三角：

```c++
auto qr1 = decompose_qr_householder(m);  // Householder 变换，数值稳定（推荐）
auto qr2 = decompose_qr_givens(m);       // Givens 旋转
auto qr3 = decompose_qr_schmitd(m);      // Gram-Schmidt 正交化

// 针对特殊结构的快速版本（配合 reduce_* 使用）
auto qr4 = decompose_qr_on_hessenberg(hess.get_reduce());
auto qr5 = decompose_qr_on_tridiagonal(trid.get_reduce());

fmat3x3 check = qr1.get_q() * qr1.get_r();   // 还原 m
```

## 特征分解：对称方阵

满足 `m = V·diag(λ)·Vᵀ`，`V` 的每一列是一个特征向量：

```c++
fmat3x3 sym { { 4.f, 1.f, 0.f }, { 1.f, 3.f, -1.f }, { 0.f, -1.f, 2.f } };

auto edv = decompose_edv_jacobi(sym);    // Jacobi 迭代
auto edv2 = decompose_edv_shiftqr(sym);   // 带位移 QR 迭代

edv.get_value();    // 特征值向量 vec<3, float>
edv.get_vector();   // 特征向量矩阵，第 i 列对应 value[i]
```

> 仅支持**对称**方阵。物理中的惯量张量、协方差矩阵等都满足此条件。

## SVD：任意规模矩阵

满足 `m = U·diag(s)·Vᵀ`：

```c++
fmat3x3 m { { 1.f, 2.f, 3.f }, { 4.f, 5.f, 6.f }, { 7.f, 8.f, 10.f } };

auto svd = decompose_svd(m);
svd.get_u();    // mat<Col, Col, T>（左正交矩阵）
svd.get_s();    // vec<min(Row, Col), T>（奇异值，降序）
svd.get_vt();   // mat<Row, Row, T>（右正交矩阵的转置）
```

典型应用：最小二乘解、伪逆、秩与条件数估计。

## 仿射分解：模型矩阵的逆向工程

把一个复合变换矩阵拆回平移、旋转、剪切、缩放四步：

```c++
// 正向：affine → model
faffine3d affine {};
affine.translate(2.f, 1.f, -3.f)
      .rotate(fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f)))
      .scale(2.f, 2.f, 4.f);
fmat4x4 model;
affine >> model;

// 逆向：model → 四个分量矩阵，满足 model = translate·rotate·shear·scale
auto comp = decompose_affine(model);
comp.get_translate();   // 平移矩阵
comp.get_rotate();     // 旋转矩阵
comp.get_shear();      // 剪切矩阵
comp.get_scale();      // 缩放矩阵
```

适合从动画数据或外部模型文件中还原变换通道。

## 精度提示

- 分解函数要求浮点基类型；
- 全部为迭代算法（内部有最大迭代次数保护），结果与输入条件数相关；
- 验证分解正确性的最直接方式是乘回去，再对差矩阵的迹做带容差比较（compare 函数只有标量与向量重载，矩阵须先归约）：

```c++
auto lu = decompose_lu_doolittle(m);
equal_zero(trace(lu.get_l() * lu.get_u() - m), 1e-4f)   // true
```

## API 参考

- [matrix 函数（全部分解签名）](/docs/function?item=matrix)
