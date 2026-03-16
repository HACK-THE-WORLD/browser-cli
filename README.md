# browser-cli

Browser automation CLI using Chrome DevTools Protocol. Connects directly to Chrome via CDP without requiring Node.js or Playwright.

## Prerequisites

Chrome must be running with remote debugging enabled:

```bash
google-chrome-stable --remote-debugging-port=9222
```

## Usage

### Get information

```bash
browser-cli get title                  # Get page title
browser-cli get url                    # Get current URL
browser-cli get text [selector]        # Get element/page text
browser-cli get html <selector>        # Get innerHTML
browser-cli get value <selector>       # Get input value
browser-cli get attr <selector> <name> # Get attribute
browser-cli get count <selector>       # Count matching elements
```

### Tab management

```bash
browser-cli tabs list                  # List open tabs
```

### JavaScript

```bash
browser-cli eval "document.title"      # Run JavaScript
```

### Global options

```bash
browser-cli --port 9222 ...            # CDP port (default: 9222)
browser-cli --json ...                 # JSON output
```

## Example

```bash
# Start Chrome with remote debugging
google-chrome-stable --remote-debugging-port=9222 &

# Run JavaScript and query page information
browser-cli get title
browser-cli get url
browser-cli eval "document.title"
browser-cli tabs list
```

## License

MIT
