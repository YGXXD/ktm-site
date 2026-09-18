# ktm guide: 坐标变换实战

本节聚焦渲染管线中的相机与投影矩阵，并说清 ktm 中"左手系 / 右手系"的选择问题。

## 变换矩阵家族

ktm 提供两类矩阵函数（头文件 `<ktm/function/matrix.h>`）：

- **2D**（返回 `mat<3, 3, T>`）：`rotate2d`、`rotate2d_point`、`rotate2d_from_to`、`translate2d`、`scale2d`、`shear2d_x`、`shear2d_y`；
- **3D**（返回 `mat<4, 4, T>`）：`rotate3d_x/y/z`、`rotate3d_axis`、`rotate3d_from_to`、`rotate3d_any_axis`、`translate3d`、`scale3d`、`shear3d_x/y/z`。

```c++
const float quarter = 0.25f * pi<float>;

fmat4x4 t = translate3d(fvec3(1.f, 2.f, 3.f));       // 平移
fmat4x4 r = rotate3d_y(half_pi<float>);             // 绕 Y 轴旋转
fmat4x4 s = scale3d(fvec3(2.f, 2.f, 4.f));          // 缩放
fmat4x4 a = rotate3d_any_axis(quarter, fvec3(1.f),   // 绕任意直线（过轴上一点）
                               fvec3(0.f, 0.f, 1.f));
```

> 与 `affine2d` / `affine3d` 的关系：这些函数生成的是"单步变换矩阵"，适合直接拼矩阵；而仿射类型适合链式组装模型变换后一次性输出。两者可自由混用，如 `affine << rotate3d_y(0.5f)`。

## 相机：观察矩阵

```c++
// 已知注视目标（最常用）
fmat4x4 view = look_at_lh(fvec3(10.f, 10.f, 10.f),   // 相机位置
                          fvec3(),                    // 注视目标（原点）
                          fvec3(0.f, 0.f, 1.f));       // 上方向

// 已知前向方向
fmat4x4 view2 = look_to_lh(eye_pos, direction, up);
```

`look_at_*` 内部即 `look_to_*(eye, normalize(focus - eye), up)`。

## 投影：透视 / 正交 / 视景体

```c++
// 透视：垂直视场角(弧度)、宽高比、近平面、远平面
fmat4x4 proj = perspective_lh(0.5f * half_pi<float>, 16.f / 9.f, 0.1f, 100.f);

// 正交：左右上下边界 + 近远平面
fmat4x4 orth = ortho_lh(-1.f, 1.f, 1.f, -1.f, 0.1f, 100.f);

// 视景体：不对称透视（斜投影、阴影矩阵等场景）
fmat4x4 fr = frustum_lh(-1.f, 1.f, 1.f, -1.f, 0.1f, 100.f);
```

## 左手系还是右手系

带 `_lh` / `_rh` 后缀的函数各有一份，区别在于：

| 项 | `_lh`（左手系） | `_rh`（右手系） |
|:-|:-|:-|
| 前向约定 | 以 `+direction` 为前向 | 以 `−direction` 为前向 |
| 深度范围 | [0, 1] | [−1, 1] |
| 典型 API | DirectX / Metal / WGPU | OpenGL / Vulkan |

选择建议：

- 按目标图形 API 的惯用约定选（见上表）；
- 同一场景内 **view 与 projection 必须使用同一手系**，否则深度与朝向会错乱；
- ktm 的矩阵乘法与列向量约定：`clip = projection * view * model * vec4(pos, 1)`。

## 组装 MVP

```c++
fvec3 pos { 5.f, -5.f, 1.f };

// 1. 模型：平移 → 旋转 → 缩放
faffine3d affine {};
affine.translate(2.f, 1.f, -3.f)
      .rotate(fquat::angle_axis(half_pi<float>, fvec3(0.f, 0.6f, 0.8f)))
      .scale(2.f, 2.f, 4.f);

fmat4x4 model;
affine >> model;

// 2. 相机与投影
fmat4x4 view = look_at_lh(fvec3(10.f, 10.f, 10.f), fvec3(), fvec3(0.f, 0.f, 1.f));
fmat4x4 projection = perspective_lh(0.5f * half_pi<float>, 16.f / 9.f, 0.1f, 100.f);

// 3. 顶点变换
fmat4x4 mvp = projection * view * model;
fvec4 clip = mvp * fvec4(pos, 1.f);
```

## 逆变换：从屏幕回到世界

```c++
fmat4x4 inv_vp = inverse(view) * inverse(projection);   // 或 inverse(projection * view)

// NDC 点 → 世界点
fvec4 world = inv_vp * fvec4(ndc_x, ndc_y, ndc_z, 1.f);
world /= world.w;   // 透视除法
```

## 2D 场景

2D 变换矩阵直接用 `mat<3, 3>`，配合 `fvec3(x, y, 1)` 齐次坐标：

```c++
const float quarter = 0.25f * pi<float>;

fmat3x3 m = translate2d(fvec2(100.f, 50.f))
          * rotate2d(quarter)
          * scale2d(fvec2(2.f, 2.f));

fvec3 p = m * fvec3(10.f, 20.f, 1.f);   // p.xy 即变换后坐标
```

绕指定点旋转用 `rotate2d_point`，一步完成"平移到原点 → 旋转 → 平移回去"。

## API 参考

- [matrix 函数（含全部变换矩阵签名）](/docs/function?item=matrix)
- [affine2d 类型](/docs/affine?item=affine2d_type) / [affine3d 类型](/docs/affine?item=affine3d_type)
