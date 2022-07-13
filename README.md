## Introduction
A **lightweight** Rust implementaion of telegraph api.

**Telegra.ph** is a minimalist publishing tool that allows you to create richly formatted posts and push them to the Web in just a click. Telegraph posts also get beautiful [Instant View](https://telegram.org/blog/instant-view) pages on Telegram.

To maintain the purity of the basic interface, we launched the @Telegraph bot for those who require advanced features. This bot can help you manage your articles across any number of devices and get page view statistics for any Telegraph page.


Anyone can enjoy the simplicity of Telegraph publishing, not just Telegram users. For this reason, all developers are welcome to use this Telegraph API to create bots like @Telegraph for any other platform, or even standalone interfaces.

## Notes
 All queries to the Telegraph API must be served over HTTPS and should be presented in this form: `https://api.telegra.ph/%method%.`

If a path parameter is present, you can also use this form: https://api.telegra.ph/%method%/%path%.
