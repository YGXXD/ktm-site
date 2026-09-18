# ktm guide: 安装与接入

## 环境要求

| 项 | 要求 |
|:-|:-|
| 编译器 | clang、gcc 或 MSVC（cl） |
| C++ 标准 | C++17 及以上（`ktm/setup.h` 中有编译期检查，低于 C++17 直接报错） |
| 平台 | 无限制，SIMD 加速自动适配 x86 / ARM / WASM |

## 方式一：CMake 安装（推荐）

ktm 提供了标准的 CMake 安装脚本，可将头文件安装到系统目录：

```shell
# unix
git clone https://github.com/YGXXD/ktm.git
cd ktm
mkdir build && cd build
cmake ..
sudo make install

# windows
cmake -S . -B ./build
cmake --install ./build --config Release
```

安装后在你的项目中通过 `find_package` 接入：

```cmake
# CMakeLists.txt
add_executable(exemple main.cpp)

find_package(ktm REQUIRED)
target_link_libraries(exemple PUBLIC ktm::ktm)
```

然后代码中直接包含总头文件即可：

```c++
#include <ktm/ktm.h>
```

## 方式二：直接引入源码（head-only）

ktm 是 head-only 库，`ktm/ktm/` 目录就是全部源码，把它拷贝或克隆为子目录后，只需把仓库根目录加入头文件搜索路径：

```cmake
# 假设 ktm 仓库位于 third_party/ktm
target_include_directories(exemple PRIVATE third_party/ktm)
```

或者直接用编译器参数：

```shell
clang++ -std=c++17 -I path/to/ktm main.cpp
```

> 注意：`-I` 指向的是含有 `ktm/ktm.h` 的目录（即仓库根目录），这样 `#include <ktm/ktm.h>` 才能被找到。

## 头文件组织

按需包含可以显著降低编译开销，各模块头文件如下：

| 头文件 | 内容 |
|:-|:-|
| `<ktm/ktm.h>` | 全部（类型 + 函数） |
| `<ktm/type_vec.h>` | 仅向量类型 |
| `<ktm/type_mat.h>` | 仅矩阵类型 |
| `<ktm/type_quat.h>` | 仅四元数类型 |
| `<ktm/type_comp.h>` | 仅复数类型 |
| `<ktm/type_affine.h>` | 仅仿射变换类型 |
| `<ktm/function/common.h>` | 通用数学函数 |
| `<ktm/function/geometric.h>` | 几何函数 |
| `<ktm/function/compare.h>` | 带容差比较 |
| `<ktm/function/random.h>` | 随机数 |
| `<ktm/function/matrix.h>` | 矩阵函数（代数 / 变换 / 分解） |
| `<ktm/function/quaternion.h>` | 四元数函数 |
| `<ktm/function/complex.h>` | 复数函数 |
| `<ktm/ktm_op.h>` | `ktm_op_madd` / `ktm_op_smadd` 融合乘加（独立于命名空间，全局可用） |

## 验证安装

用一个最小程序验证环境是否就绪：

```c++
#include <ktm/ktm.h>
#include <iostream>

int main()
{
    ktm::fvec3 v { 1.f, 2.f, 3.f };
    std::cout << ktm::dot(v, v) << std::endl;  // 输出 14
    return 0;
}
```

能正确编译并输出 `14`，说明接入成功。接下来请看[快速上手](/docs/guide?item=quickstart)。
