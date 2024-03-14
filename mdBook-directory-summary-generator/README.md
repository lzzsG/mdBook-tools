#  mdBook目录生成器使用说明

## 简介

该Python脚本自动为mdBook项目生成`SUMMARY.md`文件。它递归扫描指定目录及其所有子目录中的Markdown文件，并生成与mdBook要求相兼容的结构化目录。该工具通过自动更新目录内容，为大型文档项目的管理提供便利，节省时间并减少错误。

### 特性

- **自动目录生成：** 基于指定目录中存在的Markdown文件，创建反映mdBook项目结构的`SUMMARY.md`文件。
- **目录排除：** 允许从目录中排除特定目录，适用于忽略非文档文件夹。
- **排序选项：** 提供自然排序选项，确保文件和文件夹名称的数字顺序遵循人类直觉（例如，“1”在“10”之前）。

## 环境要求

- Python 3.x：脚本使用Python编写，需要Python 3.x运行环境。
- 操作系统：支持任何能运行Python的操作系统，包括Windows、macOS和Linux。

## 安装

无需安装。确保系统已安装Python 3.x，下载脚本即可使用。

## 使用方法

### 第1步：调整脚本参数

用文本编辑器打开脚本，修改`src_directory`变量，使其指向存储Markdown文件的顶级目录。如果有希望从目录中排除的目录，将它们添加到`ignore_dirs`列表中。

示例：

```
pythonCopy codesrc_directory = "path/to/your/markdown_files"
ignore_dirs = ["ignore_this_folder", "images", "examples"]
```

### 第2步：运行脚本

使用Python运行脚本。打开终端或命令提示符，导航到包含脚本的目录，并执行：

```
shCopy code
python path/to/script.py
```

### 第3步：将目录集成到您的mdBook项目中

运行脚本后，会在指定的输出位置创建`SUMMARY.md`文件。将`SUMMARY.md`文件及源目录的内容（确保遵循目录中概述的结构）复制到mdBook项目的`src`目录中。

## 定制化

- **忽略特定目录：** 修改`ignore_dirs`列表，包含您希望从目录中排除的任何目录。
- **排序：** 默认情况下，脚本按字母顺序对项目进行排序。在调用`create_summary_file`时，设置`use_natural_sort=True`以启用自然排序。

## 结论

该脚本简化了更新和维护mdBook项目的`SUMMARY.md`过程。通过自动生成目录，确保您的文档始终与项目结构同步更新。