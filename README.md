# Life Stats

A simple web app for tracking personal life/faith stats from several data sources - Bible memorization from Anki, Bible reading from KOReader, and prayer time from my custom prayer app Proseuche. This project as a whole is very specific to my personal workflows, with things like specific Anki deck names and card types hardcoded. So it probably wouldn't be useful for someone else to directly use. But I like open-sourcing things I work on to share and showcase what I build and in case any specific bits might be useful, such as the Anki database querying.

### My AI Coding Policy

I have used Claude Code to work on this codebase. I do not do vibe coding; Claude Code is able to write around 90% of the code, but I am actively engaging with the tool to plan and develop feature-by-feature, and am actively reviewing and refining what it produces. I take responsibility for every commit and line of code that I ship, and will review all code an AI tool produces to ensure it is correct and that I understand how it works.

## Workspace Structure

This is a Rust workspace containing multiple crates:

- **[ankistats](./ankistats/)** - Anki Bible verse memorization statistics tracker with web API and CLI

## License

This project is licensed under the [GNU Affero General Public License v3.0 or later](https://www.gnu.org/licenses/agpl-3.0.en.html).
