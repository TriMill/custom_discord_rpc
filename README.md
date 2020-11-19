# custom_discord_rpc
`custom_discord_rpc` uses the Discord RPC library for Rust to allow for fully customizable Rich Presence messages.

## Installation
Installation instructions apply to GNU/Linux, but should be similar for other operating systems

After [installing Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), build the project with `cargo build --release` and copy the following files:
 - `target/release/custom_discord_rpc` to `/usr/local/bin` (for all users) or `~/.local/bin` (for just you)
 - `config/config.yml` to a configuration directory (recommended is `~/.local/share`)

Create a [Discord application](https://discord.com/developers/applications). The name will be displayed in your status and profile.
 
Uncomment the `client_id` line from `config.yml` and add the client ID of the application you just created. Fill out the remaining fields with your custom Rich Presence message.

Start the program by running `custom_discord_rpc <config>`, where `<config>` is the path to your configuration file

## config.yml
| Field           | Value                                                             | Default   |
| --------------- | ----------------------------------------------------------------- | --------- |
| `client_id`     | The Client ID of your Discord application                         | Required  |
| `interval`      | How often (in ms) `config.yml` is read and the status is updated  | 15000     |
| `details`       | The first line of the Rich Presence text                          | None      |
| `state`         | The second line of the Rich Presence text                         | None      |
| `image`         | An image to display next to the text                              | None      |
| `tooltip`       | The tooltip to display when hovering over the image               | None      |
| `small_image`   | A second, smaller image displayed in the corner of the main image | None      |
| `small_tooltip` | The tooltip for the second image                                  | None      |

Images must be uploaded to your Discord application's assets page.
