import os
import re

def natural_sort_key(s):
    """
    为给定字符串生成一个自然排序键。
    
    :param s: 要排序的字符串
    :return: 排序键
    """
    return [int(text) if text.isdigit() else text.lower() for text in re.split('([0-9]+)', s)]

def generate_summary(directory, base_path="", ignore_dirs=[], level=0, use_natural_sort=False):
    """
    递归地遍历给定目录，生成mdBook的SUMMARY.md文件内容，忽略非.md文件和指定的文件夹。

    :param directory: 要遍历的目录路径
    :param base_path: 相对于基础目录的路径，用于生成相对路径
    :param ignore_dirs: 要忽略的文件夹名称列表
    :param level: 当前目录的层级（用于缩进）
    :param use_natural_sort: 是否使用自然排序
    :return: SUMMARY.md的内容（字符串）
    """
    summary_lines = []
    items = [item for item in os.listdir(directory) if item not in ignore_dirs]
    
    # 根据是否使用自然排序进行排序
    if use_natural_sort:
        items.sort(key=natural_sort_key)
    else:
        items.sort(key=lambda x: x.lower())
    
    indent = '    ' * level

    for item in items:
        path = os.path.join(directory, item)
        if os.path.isdir(path):
            # 为文件夹添加一个标题，并递归处理该文件夹
            summary_lines.append(f"{indent}- [{item}]()")
            new_base = os.path.join(base_path, item)
            summary_lines += [generate_summary(path, new_base, ignore_dirs, level + level + 1, use_natural_sort)]
        elif item.endswith(".md"):
            # 为Markdown文件添加一个条目，保留相对路径
            link = os.path.join(base_path, item).replace('\\', '/')
            name = os.path.splitext(item)[0]
            summary_lines.append(f"{indent}- [{name}]({link})")

    return '\n'.join(summary_lines)

def create_summary_file(src_directory, output_file="SUMMARY.md", ignore_dirs=[], use_natural_sort=False):
    """
    创建SUMMARY.md文件。

    :param src_directory: 包含Markdown文件的源目录
    :param output_file: 输出的SUMMARY.md文件路径
    :param ignore_dirs: 要忽略的文件夹名称列表
    :param use_natural_sort: 是否使用自然排序
    """
    summary_content = generate_summary(src_directory, "", ignore_dirs, 0, use_natural_sort)
    with open(output_file, "w", encoding="utf-8") as f:
        f.write(summary_content)
    print(f"SUMMARY.md has been created at {output_file}")

# 调整以下变量以适应您的项目结构
src_directory = "C:/pyth/to/your/folder"  # 您存放Markdown文件的顶级目录
ignore_dirs = ["ignore_this_folder", "figs", "examples"]  # 要忽略的文件夹名列表
# 在调用时选择是否启用自然排序
create_summary_file(src_directory, ignore_dirs=ignore_dirs, use_natural_sort=True)

