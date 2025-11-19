[Daniel Lemire's](https://lemire.me/blog/) blog is full of common problems and (very) efficient solutions (most often SIMD-based) encountered in the data processing domain.
Below is an attempt at a "categorization" for easy access. In addition, I'm reimplementing a couple of them in Rust to get the hang of they work. I'm using ARM NEON intrinsics, which are built-in my MacBook Pro. I'm capturing the result of the benchmark on my laptop as a comment at the top of the source files.

- [String Transformations](#string-transformations)
- [Character & Pattern Detection](#character--pattern-detection)
- [Encoding & Transcoding](#encoding--transcoding)
- [Parsing Structured Data](#parsing-structured-data)
- [Numeric Operations](#numeric-operations)
- [Filtering & Selection](#filtering--selection)
- [Aggregation & Analysis](#aggregation--analysis)

---

# String Transformations

### AVX-512
* [ASCII to lower case](https://lemire.me/blog/2024/08/03/converting-ascii-strings-to-lower-case-at-crazy-speeds-with-avx-512)
* [Removing chars from strings](https://lemire.me/blog/2022/04/28/removing-characters-from-strings-faster-with-avx-512)
* [Escaping strings](https://lemire.me/blog/2022/09/14/escaping-strings-faster-with-avx-512)

### SVE (ARM)
* [Trimming spaces from strings](https://lemire.me/blog/2023/03/10/trimming-spaces-from-strings-faster-with-sve-on-an-amazon-graviton-3-processor/)

---

# Character & Pattern Detection

### SWAR
* [Detect control characters, quotes, and backslashes](https://lemire.me/blog/2025/04/13/detect-control-characters-quotes-and-backslashes-efficiently-using-swar/)
* [JSON escapable characters](https://lemire.me/blog/2025/04/13/detect-control-characters-quotes-and-backslashes-efficiently-using-swar/)

### NEON (ARM)
* [Locating identifiers](https://lemire.me/blog/2023/09/04/locating-identifiers-quickly-arm-neon-edition)

### SIMD (General)
* [Recognizing string prefixes](https://lemire.me/blog/2023/07/14/recognizing-string-prefixes-with-simd-instructions/)
* [Identifying sequence of digits in strings](https://lemire.me/blog/2018/09/30/quickly-identifying-a-sequence-of-digits-in-a-string-of-characters)
* [Scan HTML (Chrome)](https://lemire.me/blog/2024/06/08/scan-html-faster-with-simd-instructions-chrome-edition)
* [Scan HTML (.NET C#)](https://lemire.me/blog/2024/07/05/scan-html-faster-with-simd-instructions-net-c-edition)
* [Scan HTML (C/C++)](https://lemire.me/blog/2024/07/20/scan-html-even-faster-with-simd-instructions-c-and-c)

### AVX-512
* [Checking for absence of a string](https://lemire.me/blog/2022/12/15/checking-for-the-absence-of-a-string-naive-avx-512-edition)

---

# Encoding & Transcoding

### AVX-512
* [Latin-1 to UTF-8 (12 GB/s)](https://lemire.me/blog/2023/08/18/transcoding-latin-1-strings-to-utf-8-strings-at-12-gb-s-using-avx-512)
* [UTF-8 to Latin-1 (12 GB/s)](https://lemire.me/blog/2023/08/12/transcoding-utf-8-strings-to-latin-1-strings-at-12-gb-s-using-avx-512)
* [Bitset decoding](https://lemire.me/blog/2022/05/06/fast-bitset-decoding-using-intel-avx-512)

### SWAR
* [Encoding binary in ASCII](https://lemire.me/blog/2020/05/02/encoding-binary-in-ascii-very-fast)

### Other/Scalar
* [Base16 encoding](https://lemire.me/blog/2022/12/23/fast-base16-encoding)
* [Decoding Base16 sequences](https://lemire.me/blog/2023/07/27/decoding-base16-sequences-quickly)

---

# Parsing Structured Data

### Numbers

**AVX-512**
* [Parsing integers](https://lemire.me/blog/2023/09/22/parsing-integers-quickly-with-avx-512)

**SWAR**
* [String of digits to integer](https://lemire.me/blog/2022/01/21/swar-explained-parsing-eight-digits)

**Other/Scalar**
* [Floating point parsing](https://lemire.me/blog/2021/02/22/parsing-floating-point-numbers-really-fast-in-c)

### Timestamps
* [Parsing timestamps (SIMD)](https://lemire.me/blog/2023/07/01/parsing-time-stamps-faster-with-simd-instructions)

### IP
* [Parsing IP addresses](https://lemire.me/blog/2023/06/08/parsing-ip-addresses-crazily-fast/)

### Domain names
* [Domain names to wire format (prefix minimum)](https://lemire.me/blog/2023/08/10/coding-of-domain-names-to-wire-format-at-gigabytes-per-second)

---

# Numeric Operations

### AVX-512
* [Integers to decimal strings](https://lemire.me/blog/2022/03/28/converting-integers-to-decimal-strings-faster-with-avx-512)

### SWAR
* [Integers to fixed digits](https://lemire.me/blog/2021/11/18/converting-integers-to-fix-digit-representations-quickly/)

### Other
* [Packing string of digits into integer](https://lemire.me/blog/2023/07/07/packing-a-string-of-digits-into-an-integer-quickly)

---

# Filtering & Selection

### Filtering numbers
* [Filtering numbers](https://lemire.me/blog/2022/07/14/filtering-numbers-faster-with-sve-on-amazon-graviton-3-processors/)

### Duplicates
* [Removing duplicates from lists](https://lemire.me/blog/2017/04/10/removing-duplicates-from-lists-quickly/)

---

# Aggregation

### AVX-512
* [Computing UTF-8 size of Latin-1 string](https://lemire.me/blog/2023/02/16/computing-the-utf-8-size-of-a-latin-1-string-quickly-avx-edition/)

### SWAR
* [Counting matching characters in two ASCII strings](https://lemire.me/blog/2021/05/21/counting-the-number-of-matching-characters-in-two-ascii-strings)
