# txt2csv

`txt2csv` — це невеликий парсер, який перетворює напівструктуровані `.txt` файли у `.csv`.  
Він написаний на Rust із використанням бібліотеки [`pest`](https://pest.rs) для граматичного розбору тексту.

---

## 🧩 Brief Description
The `txt2csv` tool reads text files where values are separated by commas, pipes (`|`), or spaces, and converts them into properly formatted CSV files.

---

## ⚙️ Technical Description

### What is being parsed
The parser processes each line of a `.txt` file:
- Splits rows by newline characters (`\n` or `\r\n`);
- Splits each row into **fields** separated by commas (`,`);
- Trims whitespace and removes square brackets (e.g., `[QUOTE_DATE] → QUOTE_DATE`);
- Handles empty values (e.g., `,,` → empty CSV cell).

### How results are used
After parsing:
1. Each `row` becomes a CSV line.
2. Each `field` becomes a properly separated CSV cell.
3. The cleaned data are saved as a `.csv` file, which can be used for further data analysis in Excel, pandas, or SQL.
