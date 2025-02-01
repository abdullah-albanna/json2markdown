
## Metadata



- **Version**

1.0
- **Generated At**

2025-02-01T12:34:56Z
- **Description**

  A complex JSON structure for testing Markdown rendering

  Totaly not from AI


## Users



  - **Id**: 1
  - **Name**

  Alice Wonderland
  - **Email**

  alice@example.com
  - **Bio**

    Software engineer & open-source enthusiast

    Loves Rust & Linux

    Contributor to various projects.


  - **Website**

https://example.dev
  - **Settings**

    - **Theme**

    dark
    - **Notifications**

      - **Email**: true
      - **Sms**: false
  - **Posts**

      - **Id**: 101
      - **Title**

      Rust Performance Tips
      - **Content**

        Rust is fast, but there are ways to make it even faster

        Here are some tips:

        Use `#[inline(always)]` where necessary

        Prefer stack allocation over heap

        Profile your code with `perf`

        Check out the full guide here: https://rust-lang.org/performance


      - **Tags**

          - Rust
          - Performance
          - Optimization


      - **Comments**

          - **User**

          Bob
          - **Message**

          Great tips! I also found that using `rayon` can improve performance in parallel tasks.
          - **User**

          Charlie
          - **Message**

          Would love to see an example of using `unsafe` for optimization!




  - **Id**: 2
  - **Name**

  Bob Builder
  - **Email**

  bob@example.com
  - **Bio**

    Builder of things

    Rustacean in training.


  - **Website**: N/A
  - **Settings**

    - **Theme**

    light
    - **Notifications**

      - **Email**: false
      - **Sms**: true
  - **Posts**

      - **Id**: 202
      - **Title**

      Building CLI tools in Rust
      - **Tags**

          - Rust
          - CLI
          - Tools


      - **Content**

        Building CLI tools with Rust is easy with `clap` and `structopt`

        Start by adding the dependency:
```toml
[dependencies]
clap = "4.0"
```
Then define your CLI options:
```rust
use clap::{App, Arg};

fn main() {
    let matches = App::new("My CLI")
        .version("1.0")
        .author("Bob Builder <bob@example.com>")
        .about("Does awesome things")
        .arg(Arg::new("input")
             .about("The input file")
             .required(true)
             .index(1))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    println!("Processing file: {}", input);
}
```






## Stats



- **Total Users**: 2
- **Total Posts**: 2
- **Average Comments Per Post**: 1.5
- **Popular Tags**

    - Rust
    - CLI
    - Performance


- **Random Numbers**

    - 42
    - 3.14159
    - 2.71828
    - 1000000.0
    - -273.15


- **Unicode Test**

日本語, 한국어, русский, العربية, emojis 🎉🚀🔥
## Nested



- **Level 1**

  - **Level 2**

    - **Level 3**

      - **Array**

          - **Deep Key**

          deep_value
          - **Numbers**

              - 1
              - 2
              - 3
              - 4
              - 5


          - **Bools**

              - true
              - false
              - N/A


          - **Nested Obj**

            - **Key**

            value
            - **Sub Array**

                - a
                - b
                - c


          - **Deep Key**

          another_value
          - **Complex String**

            This is a long sentence

            It should be properly split by your Markdown renderer! And this? Another sentence

            Testing..

            1, 2, 3.





