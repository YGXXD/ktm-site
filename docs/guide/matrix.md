# ktm guide: 矩阵

## 模版参数与存储布局

`mat<Row, Col, T>` 的参数含义需要特别注意：

- **`Row` 是列数**，`Col` 是每列的分量个数（即通常意义上的行数），两者均须大于 1；
- 存储为**列主序**：`m[i]` 是第 i 个**列向量**（类型 `vec<Col, T>`），`m[i][j]` 是第 i 列中的第 j 行元素。

```c++
fmat3x3 m { { 1.f, 2.f, 3.f },     // 第 0 列 (m00, m10, m20)
            { 4.f, 5.f, 6.f },     // 第 1 列 (m01, m11, m21)
            { 7.f, 8.f, 10.f } };  // 第 2 列 (m02, m12, m22)

fvec3 col0 = m[0];    // 第 0 列 (1, 2, 3)
float m12 = m[1][2];  // 第 1 列的第 2 行元素 = 6
```

这与 OpenGL 的 `GLSL` 一致：`m[i]` 取列，`m[i][j]` 即 `mat[i][j]`（第 j 行第 i 列）。

常用别名：`fmat2x2` ~ `fmat4x4`、`dmat2x2` ~ `dmat4x4`、`smat*`、`umat*`，以及模版别名 `fmat<Row, Col>`。

## 构造

```c++
fmat3x3 a;                        // 全 0 矩阵
fmat3x3 b(fvec3(1.f, 2.f, 3.f), fvec3(4.f, 5.f, 6.f), fvec3(7.f, 8.f, 9.f));  // 列向量变参
fmat3x3 c { { 1.f, 0.f, 0.f }, { 0.f, 1.f, 0.f }, { 0.f, 0.f, 1.f } };          // 列表初始化
```

## 静态构造方法

```c++
fmat3x3 eye = fmat3x3::from_eye();                     // 单位矩阵（方阵）
fmat3x3 diag = fmat3x3::from_diag(fvec3(2.f, 3.f, 4.f));  // 对角矩阵
fmat2x2 byrow = fmat2x2::from_row(fvec2(1.f, 2.f), fvec2(3.f, 4.f));  // 按行构造
```

`from_row` 接受 Col 个"行向量"（每个含 Row 个元素），在按数学书写习惯（行优先）输入矩阵时很方便，内部会转置为列主序存储。

## 乘法规则

矩阵乘法支持三种组合，维度约束如下：

| 表达式 | 约束 | 结果 | 含义 |
|:-|:-|:-|:-|
| `m * v` | v 的分量数 = m 的**列数**（Row） | `vec<Col, T>` | 列向量的线性组合 `Σv[i]·m[i]` |
| `v * m` | v 的分量数 = m 的**行数**（Col） | `vec<Row, T>` | 行向量左乘 |
| `m1 * m2` | m2 的行数 = m1 的列数 | `mat<U, Col>` | 矩阵乘积 |
| `m1 *= m2` | 仅同规模**方阵** | 自身引用 | `m1 = m1 * m2` |

```c++
fmat4x4 mvp = projection * view * model;  // 常规矩阵连乘
fvec4 clip = mvp * fvec4(pos, 1.f);       // 变换顶点
```

## 代数运算

```c++
transpose(m)     // 转置，任意规模
diagonal(m)       // 对角线向量（方阵）
trace(m)          // 迹 = 对角线元素之和（方阵）
determinant(m)    // 行列式（方阵）
inverse(m)        // 逆矩阵（方阵，浮点基类型；要求行列式非零）
```

验证求逆：

```c++
fmat3x3 m { { 1.f, 2.f, 3.f }, { 4.f, 5.f, 6.f }, { 7.f, 8.f, 10.f } };
fmat3x3 check = m * inverse(m);   // 得到单位矩阵
```

## 逐分量运算

矩阵同样支持与同型矩阵的 `+ -`、与标量的 `+ - * /`、复合赋值、一元取反、字典序比较与流 IO，全部为**逐分量**操作（注意 `+` / `-` 不是矩阵意义上的运算之外的东西——它们就是逐分量加减）：

```c++
fmat2x2 a { { 1.f, 0.f }, { 0.f, 1.f } };
fmat2x2 b = a * 2.f + 1.f;   // 逐分量
```

融合乘加：`ktm_op_madd(x, y, s)` / `ktm_op_smadd(x, y, s)`，同向量版本。

## 与图形 API 的对接

列主序内存布局意味着 `m.data()` 可以直接按列主序上传：

- **OpenGL / Vulkan / Metal (column-major)**：`m.data()` 原样可用；
- **DirectXMath (row-major)**：注意 ktm 的 `m[i][j]` 是"第 j 行第 i 列"，与 HLSL 中 `mul(M, v)` 语义一致时可原样使用；若你的管线按行主序解释内存，请先 `transpose`。

## API 参考

- [mat 类型](/docs/matrix?item=mat_type)
- [mat 静态方法](/docs/matrix?item=mat_static)
- [mat 运算符](/docs/matrix?item=mat_operator)
- [matrix 函数（代数 / 变换 / 分解）](/docs/function?item=matrix)
