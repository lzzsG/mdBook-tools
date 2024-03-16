# mdBook工具集 

## 仓库简介

此仓库收集了一系列自制的mdBook工具，旨在提升mdBook项目的管理效率和视觉体验。

### 项目列表

1. **目录生成器** - 一个Python脚本，用于自动扫描指定目录下的Markdown文件并生成mdBook所需的`SUMMARY.md`文件。它支持目录排除和自然排序功能，简化了目录更新过程。
2. **自定义主题替换** - 一套自定义的mdBook主题，包含黑色主题风格、自定义导航和CSS效果调整。只需将该主题复制到mdBook项目文件夹，即可轻松更换默认主题。
3. **funcs for rustlings-notebook** - 为构建一个基于`mdbook`的`rustlings-notebook`教程而设计的Rust程序，自动化了编号分配、文件结构生成和目录汇总等任务

## 使用说明

### 1.目录生成器

- 需要Python 3环境。
- 调整脚本内的参数，指定Markdown文件的存放目录和需要排除的文件夹。
- 运行脚本后，将生成的`SUMMARY.md`文件复制到mdBook项目的`src`目录。

### 2.自定义主题替换

- 将整个`theme`文件夹复制到mdBook项目目录。
- 自定义主题包含以下修改：
  - **黑色主题风格**：在`theme/css/variables.css`文件中定义。
  - **自定义导航**：在`theme/index.hbs`文件中实现。
  - **CSS效果调整**：通过`theme/css/chrome.css`文件进行。

#### 调整或删除主题

- **删除主题**：直接将`theme`文件夹从项目中删除，mdBook将使用默认主题。
- **修改主题和效果**：在`theme`文件夹中更改CSS和.hbs文件，根据需要调整导航样式和其他视觉效果。

### 3.funcs for rustlings-notebook

- func1: 读取练习的信息并生成对应的Markdown文件，保留对应文件夹层级结构。每个文件包含练习的编号、名称、路径和提示信息。
- func2: 为每个练习自动添加一个编号。
- func3: 读取练习信息并生成一个顺序目录结构的Markdown文件，包含所有练习的链接列表，链接指向每个练习对应的Markdown文件，方便作为目录跳转。
- 执行方式`cargo run --bin funcNum`
 
---
后续可能会增加更多实用工具，以进一步扩展mdBook的功能和灵活性。
