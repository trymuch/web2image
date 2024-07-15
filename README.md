# README

## 1.概述

一个可以截图网页，并在截取的图片上面添加一个二维码的命令行工具。通过扫码二维码图片可以获取截取的网页的URL链接。

## 2.准备

### 2.1需要用到crate

- clap: 解析命令行

- headless-chrome 可以获取网页元素或者整个页面的截图

- qrcode QRCODE编码器，可以生成二维码图片

- url 解析url并提供相关的数据结构

- anyhow 提供通用的错误处理类型

### 2.2 引入依赖

```toml
[dependencies]
anyhow = "1.0.86"
clap = { version = "4.5.9", features = ["derive"] }
headless_chrome = "1.0.10"
qrcode = "0.14.1"
url = "2.5.2"
```

## 3.思路

### 3.1 分析命令行需要的参数

创建一个命令行工具，首先需要根据这个命令行工具的用途分析命令行在终端应该如何输入命令。这需要分析命令行的结构，即命令行由什么参数 组成。

根据命令行参数前面是否带有  `-f` 或`--flag` 的标记可以分为：

- 选项参数  -- 带有前置标记，也就是该参数需要在命令行中输入其所需的值。注意：虽然是叫做选项参数，但是可能是必选的。

- 位置参数 -- 不带有前置标记，无需再命令行中输入所需的值

根据命令行参数是否可选（是否命令中是否一定要带上这个参加就才能执行）分为：

- 可选参数 -- 无需在命令行中出现

- 必选参数 -- 必须在命令行中出现，不出现命令无法正确执行

这个命令行工具一定需要一个必选的URL作为参数，可以参数可以是选项参数，也可以位置参数。这里我们将其作为位置参数就好了，即我们直接在命令行中输入URL即可。一般必选的参数如果只有一个我们可以用位置参数的形式。我们命令有一个输出结果，就是图片文件。我们的图片文件需要一个存储位置，所以需要设定输出结果的文件路径。这个可以是可选参数，如果我们不在命令行中设定输出的结果，可以默认在命令运行的当前目录存储最终的图片文件。

总之，有一个位置参数url和一个可选的选项参数output。

### 3.2定义命令行结构体，使用clap解析命令行

```rust
/// A command-line tool that can take screenshots of web pages and add a QR code on top of the captured image.
/// The URL link of the captured webpage can be obtained by scanning the QR code image.
#[derive(Debug, Parser)]
#[command(version = "0.1.0",author,about,long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// input url
    url: Url,
    /// output file
    #[arg(short, long, value_name = "FILE",value_parser = valid_path)]
    output: Option<PathBuf>,
}
```

`Cli` 结构体是该命令行应用程序的命令行的抽象，该结构体有两个字段：`url` 字段代表的是用户输入的url，`output` 字段是用户指定的输出图片的存储位置（包含路径和图片文件的名字）。`url`的类型是Url，说明其是一个必选的参数。`output` 的类型是`Option<PathBuf>` ,说明其是一个可选的参数。 `Parser` 派生宏使得`Cli` 实现了 `Parser` trait，可以调用`parse()` 方法，从而可以将用户输入的命令行解析为`Cli` 结构体。`command` 可以设定命令行程序的`version` ,`author` ,`about` ,`long_about` ，如果没有赋值，那么就会从`Cargo.toml` 文件中或者从文档注释中获取值。`arg` 属性宏可以设定用户输入参数（即Cli的字段）的相关属性。指定了`short` ,`long` 这项 属性可以使该参数变成有标志的参数。`output` 参数在命令行中输入的时候可以用 `-o output` 或者`--output output` 的形式。

`Cli` 的字段的类型本身对用户的输入进行了校验，只有符合对应类型的命令行输入才能解析为相应的类型。如果用户输入的url是不合法的，那么就不能正确的解析为Url类型，程序就会报错，并让用户输入正确的url。但是有时候用户输入了正确的内容，并且可以正确的解析为参数相应的类型，但是其他的限制导致一部分符合类型要求的输入不是合法的输入。此时需要对用户的输入校验。通过属性`value_parser` 校验用户的输入，需要自定义校验函数。比如output参数需要确保用户输入的路径中目录是存在的，如果目录不存在，就需要报错，并告诉用户输入存在的目录。另外用户指定的图片名称需要符合应用程序要求的图片格式。代码如下:

```rust
// validate output path
fn valid_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value.to_owned());
    let parent_option = path.parent();

    match parent_option {
        Some(parent) if parent.to_str().unwrap() != "" && !parent.exists() => {
            return Err("The specified parent directory does not exist.".into());
        }
        _ => {}
    }
    let extension = path.extension().and_then(|ext| {
        let ext = ext.to_str().unwrap().to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" => Some(ext),
            _ => None,
        }
    });
    match extension {
        Some(_) => Ok(path),
        None => Err("Please specify the correct format through the extension, such as png, jpg, jpeg etc.")?,
    }
}
```


