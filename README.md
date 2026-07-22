## ./fuzz

A lightweight command-line search tool for text files, written in Rust. Instead of relying on exact matches, the program evaluates each line using a custom scoring algorithm to find and rank the most relevant results.

### Installation

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd fuzz
cargo build --release
```

The compiled binary will be available at:

```text
target/release/fuzz
```

### Usage

```text
fuzz [OPTIONS] <TEXT> <FILE_PATH>
```

#### Arguments

| Argument | Description |
|----------|-------------|
| `TEXT` | Search query |
| `FILE_PATH` | Path to the text file |

#### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-t, --top` | Percentage of lines to return | `50` |
| `-l, --lines` | Display line numbers | Disabled |
| `-s, --score` | Display relevance scores | Disabled |
| `-h, --help` | Print help information | - |
| `-V, --version` | Print version information | - |


### Example

```bash
./fuzz -l -s -t 10 "lorem ipsum" lorem.txt
```

Output:

```text
1: [336.543] Lorem ipsum dolor sit amet, ex aliquip tempor irure ex et dolor consectetur. Dolore deserunt eiusmod lorem nostrud. Lorem eiusmod eiusmod excepteur. Sunt voluptate qui voluptate proident, laborum cillum adipiscing tempor cillum id magna veniam. Aliquip esse consectetur enim aute. Et nulla sit minim aliqua.
4: [120.910] Irure tempor labore proident quis. Culpa tempor aute irure sed velit culpa labore. Mollit enim irure pariatur, ea nostrud laborum dolore excepteur eu. Mollit sint eiusmod adipiscing lorem do reprehenderit tempor. Pariatur occaecat dolore adipiscing, consequat ea ipsum consequat aliqua ullamco. Id velit cillum nostrud dolor consequat. Commodo nulla veniam duis nostrud est do. Id proident voluptate ullamco in labore ipsum sed.
```
