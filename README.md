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

### Callback-style APIs

If an API returns data via callback (instead of immediate return), wrap it in a Promise and resolve inside the callback:

```bash
browser-cli eval "new Promise((resolve, reject) => {
	const timer = setTimeout(() => reject(new Error('timeout waiting callback')), 10000);
	window.xxx.register_function((data) => {
		clearTimeout(timer);
		resolve(data);
	});
	window.xxx.getUser();
})"
```

### Global options

```bash
browser-cli --port 9222 ...            # CDP port (default: 9222)
browser-cli --json ...                 # JSON output
browser-cli -t <tab> ...               # Run on a specific tab (index or real id)
browser-cli --tab-id <tab> ...         # Same as -t
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
