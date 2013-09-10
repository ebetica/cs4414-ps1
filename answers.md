Title: Problem Set 1 Answers
Author: Zeming Lin

1. User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86\_64; rv:23.0) Gecko/20100101 Firefox/23.0. "Mozilla/5.0" I assume is the company that made Firefox and there's some set of rules the webiste follows in that regard. "X11" is the display method I'm using. "Ubuntu; Linux x86\_64" means that the operating system I'm on is 32-bit Linux. "Gecko/20100101" is the display manager my web browser is using, and "Firefox/23.0" is my brower's name and version.
2. A mutable static variable presents problems in concurrency, because accessing the variable may lead to problems like race conditions. Rust is built around safe concurrency, so we have to put an "unsafe" modifier on the function that uses it.
