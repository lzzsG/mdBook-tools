### 自定义主题替换

- 将整个`theme`文件夹复制到mdBook项目目录。
- 自定义主题包含以下修改：
  - **黑色主题风格**：在`theme/css/variables.css`文件中定义。
  - **自定义导航**：在`theme/index.hbs`文件中实现。
  - **CSS效果调整**：通过`theme/css/chrome.css`文件进行。

#### 调整或删除主题

- **删除主题**：直接将`theme`文件夹从项目中删除，mdBook将使用默认主题。
- **修改主题和效果**：在`theme`文件夹中更改CSS和.hbs文件，根据需要调整导航样式和其他视觉效果。