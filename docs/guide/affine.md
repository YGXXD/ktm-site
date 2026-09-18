# ktm guide: 仿射变换

`affine2d` / `affine3d` 表示"线性变换 + 平移"的复合，是组装模型矩阵（model matrix）的最佳工具。

## 为什么用仿射变换而不是 4×4 矩阵

- **更省**：`affine3d` 内部只有 3×3 线性部分 + 3 维平移列（12 个分量），比 4×4 少 4 个；
- **更快**：复合时跳过齐次边界行列的无效计算；
- **更好写**：所有变换方法返回自身引用，链式表达变换序列。

## 数据布局

```c++
// affine3d：m 为 mat<4, 3, T>，4 个列向量
//   m[0]、m[1]、m[2] 为线性部分（旋转 / 缩放 / 剪切）
//   m[3] 为平移
faffine3d affine {};

// affine2d：m 为 mat<3, 2, T>，并提供具名成员
//   a / b / c / d 为线性部分，tx / ty 为平移
faffine2d affine2 {};
affine2.a; affine2.tx;   // 与 m[0][0]、m[2][0] 等价
```

## 构造

```c++
faffine3d a;                          // 单位变换
faffine3d b(fmat3x3::from_eye());     // 由 3×3 线性矩阵，平移为 0
faffine3d c(fmat4x4::from_eye());      // 由 4×4 齐次矩阵，取左上 3×3 与平移列

faffine2d d;                           // 单位变换
faffine2d e(fmat2x2::from_eye());     // 由 2×2 线性矩阵
faffine2d f(fmat3x3::from_eye());     // 由 3×3 齐次矩阵
faffine2d g(fmat4x4::from_eye());     // 由 4×4 矩阵，取左上 2×2 与第 3 列 xy
```

## 链式变换

```c++
fquat spin = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));

faffine3d affine {};
affine.translate(2.f, 1.f, -3.f)   // 平移（分量或向量）
       .rotate(spin)               // 四元数旋转
       .rotate_x(0.3f)             // 绕 X 轴
       .rotate_y(0.3f)              // 绕 Y 轴
       .rotate_z(0.3f)              // 绕 Z 轴
       .rotate_axis(0.5f, fvec3(0.f, 0.f, 1.f))      // 绕任意轴
       .rotate_from_to(fvec3(0.f, 0.f, 1.f), fvec3(0.f, 1.f, 0.f))  // 方向对齐
       .scale(2.f, 2.f, 4.f)       // 缩放（分量或向量）
       .shear_x(0.1f, 0.2f);       // 剪切
```

二维版本：

```c++
faffine2d affine2 {};
affine2.translate(1.f, 2.f)
         .rotate(half_pi<float>)    // 角度（内部转复数）；亦可传 fcomp
         .scale(2.f, 3.f)
         .shear_x(0.1f)            // x' = x + tan(angle)·y
         .shear_y(0.2f);           // y' = y + tan(angle)·x
```

书写顺序即应用顺序，`affine.translate(...).rotate(...)` 对应矩阵 `T·R`（先平移后旋转）。

## 复合与求逆

```c++
// 右复合另一变换或矩阵：等价 m = m * other
affine.concat(other_affine);
affine << other_affine;              // operator<< 是 concat 的语法糖
affine << fmat3x3::from_eye();
affine << fmat4x4::from_eye();

// 就地求逆（要求线性部分可逆）
affine.invert();
```

## 输出到矩阵

交给渲染管线前，把仿射变换展开为标准矩阵：

```c++
fmat4x4 model;
affine >> model;                     // 等价 affine.matrix4x4(model)

fmat3x3 linear;
affine >> linear;                     // 等价 affine.matrix3x3(linear)，丢弃平移

// 显式调用亦可
affine.matrix4x4(model);
affine.matrix3x3(linear);
```

`operator>>` 让"变换流入矩阵"的方向感很直观，这也是 README 示例中 `affine... >> model` 的写法。

## 完整示例

```c++
fvec3 vector { 5.f, -5.f, 1.f };
fquat quaternion = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));

faffine3d affine {};
fmat4x4 model;
affine.translate(2.f, 1.f, -3.f).rotate(quaternion).scale(2.f, 2.f, 4.f) >> model;

// 用 model 变换点（齐次坐标 w=1）
fvec4 transformed = model * fvec4(vector, 1.f);
```

## API 参考

- [affine2d 类型](/docs/affine?item=affine2d_type)、[affine2d 运算符](/docs/affine?item=affine2d_operator)
- [affine3d 类型](/docs/affine?item=affine3d_type)、[affine3d 运算符](/docs/affine?item=affine3d_operator)
