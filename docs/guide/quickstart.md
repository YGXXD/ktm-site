# ktm guide: 快速上手

本节用一个完整的渲染场景示例串起 ktm 的核心用法：构造向量与四元数、旋转、仿射变换、组装 MVP 矩阵。

## 完整示例

```c++
#include <ktm/ktm.h>
#include <iostream>

using namespace ktm;
using namespace std;

int main(int argv, char* argc[])
{
    // 构造向量 vector
    fvec3 vector { 5.f, -5.f, 1.f };

    // 通过旋转角和轴构建四元数 quaternion
    fquat quaternion = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));
    // 利用四元数 quaternion 旋转向量 vector
    cout << quaternion * vector << endl;

    // 通过四元数 quaternion 构建旋转矩阵 rotate
    fmat3x3 rotate = quaternion.matrix3x3();
    // 利用矩阵 rotate 旋转向量 vector
    cout << rotate * vector << endl;

    // 构建仿射变换 affine, 定义矩阵 model
    faffine3d affine { }; fmat4x4 model;
    // 仿射变换 affine 进行平移旋转缩放后, 输入到矩阵 model
    affine.translate(2.f, 1.f, -3.f).rotate(quaternion).scale(2.f, 2.f, 4.f) >> model;
    // 利用矩阵 model 变换向量 vector
    cout << model * fvec4(vector, 1.f) << endl;

    // 构建视口变换矩阵 view
    fmat4x4 view = look_at_lh(fvec3(10.f, 10.f, 10.f), fvec3(), fvec3(0.f, 0.f, 1.f));
    // 构建投影变换矩阵 projection
    fmat4x4 projection = perspective_lh(0.5f * half_pi<float>, 16.f / 9.f, 0.1f, 100.f);
    // 利用矩阵 model, view, projection 构建 mvp 变换矩阵
    cout << projection * view * model << endl;

    return 0;
}
```

## 逐段解读

### 1. 向量：图形学的基本单位

```c++
fvec3 vector { 5.f, -5.f, 1.f };
```

`fvec3` 是 `vec<3, float>` 的别名，花括号列表按 x、y、z 顺序初始化分量。向量支持全部算术运算符、下标访问与流输出：

```c++
fvec3 a { 1.f, 2.f, 3.f };
fvec3 b = a * 2.f + fvec3(1.f);   // 逐分量运算
float d = dot(a, b);              // 点积
fvec3 n = normalize(a);            // 归一化
```

详见[向量](/docs/guide?item=vector)一章。

### 2. 四元数：三维旋转的推荐表达

```c++
fquat quaternion = fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f));
cout << quaternion * vector << endl;
```

- `angle_axis(角, 轴)` 构造绕任意单位轴旋转指定弧度的四元数；
- `quaternion * vector` 直接把三维向量旋转到新位置，无需先转成矩阵；
- 轴必须是**单位向量**，角度使用**弧度**。

### 3. 矩阵：批量变换与管线对接

```c++
fmat3x3 rotate = quaternion.matrix3x3();
cout << rotate * vector << endl;
```

四元数与旋转矩阵可以随时互转：`matrix3x3()` 得到 3×3 旋转矩阵，`matrix4x4()` 得到齐次 4×4 矩阵；反向转换用 `fquat::from_matrix(m)`。当同一个旋转要作用到大量向量时，先转矩阵再乘通常更快。

> ktm 的矩阵是**列主序**：`m[i]` 是第 i 列，`m[i][j]` 是第 i 列中的第 j 行元素。这与 OpenGL 一致。

### 4. 仿射变换：链式组装模型矩阵

```c++
faffine3d affine { }; fmat4x4 model;
affine.translate(2.f, 1.f, -3.f).rotate(quaternion).scale(2.f, 2.f, 4.f) >> model;
```

`affine3d` 内部只存 3×3 线性部分 + 平移列（比 4×4 矩阵省空间、乘法更快），所有变换方法都返回自身引用，因此可以链式书写。`>> model` 把仿射变换展开输出到 4×4 矩阵（`operator>>` 等价于 `matrix4x4(model)`）。

变换顺序即书写顺序：先平移、再旋转、最后缩放，对应 `model = T·R·S`。

### 5. 相机与投影：MVP

```c++
fmat4x4 view = look_at_lh(fvec3(10.f, 10.f, 10.f), fvec3(), fvec3(0.f, 0.f, 1.f));
fmat4x4 projection = perspective_lh(0.5f * half_pi<float>, 16.f / 9.f, 0.1f, 100.f);
cout << projection * view * model << endl;
```

- `look_at_lh(相机位置, 注视目标, 上方向)` 生成观察矩阵；
- `perspective_lh(垂直视场角, 宽高比, 近平面, 远平面)` 生成透视投影矩阵；
- `_lh` / `_rh` 后缀分别表示左手系 / 右手系，按你的图形 API 选择（如 DirectX 惯用左手系 [0,1] 深度，OpenGL 惯用右手系 [-1,1] 深度）；
- 最终顶点变换顺序：`clip = projection * view * model * position`。

## 运行结果

```
4.6 2.68 -4.76
4.6 2.68 -4.76
12.4 7.32 -11.24 1
-1.5364 -3.94239 -0.231171 -0.23094 ...
```

四元数旋转与矩阵旋转结果一致；MVP 矩阵可直接送入渲染管线。

## 下一步

- 各类型的深入用法：[向量](/docs/guide?item=vector)、[矩阵](/docs/guide?item=matrix)、[四元数](/docs/guide?item=quaternion)、[复数](/docs/guide?item=complex)、[仿射变换](/docs/guide?item=affine)；
- 相机、投影与左右手系的完整说明：[坐标变换实战](/docs/guide?item=transform)；
- 每个接口的精确签名：左侧 API 参考分组。
