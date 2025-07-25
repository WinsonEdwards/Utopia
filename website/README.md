# Utopia Programming Language Website

A modern, responsive website for promoting the Utopia programming language project.

## Features

- **Modern Design**: Clean, professional design with smooth animations
- **Responsive Layout**: Works perfectly on desktop, tablet, and mobile devices
- **Interactive Elements**: 
  - Smooth scrolling navigation
  - Interactive code examples with language switching
  - Copy-to-clipboard functionality for code blocks
  - Animated statistics counters
  - Newsletter subscription form
- **Cross-Browser Compatible**: Works on all modern browsers

## Files Structure

```
website/
├── index.html          # Main HTML file
├── styles.css          # CSS styles and responsive design
├── script.js           # JavaScript functionality
└── README.md           # This file
```

## Running the Website

### Option 1: Python HTTP Server
```bash
cd website
python3 -m http.server 8000
```
Then open http://localhost:8000 in your browser.

### Option 2: Node.js HTTP Server
```bash
cd website
npx http-server -p 8000
```

### Option 3: Live Server (VS Code Extension)
Install the "Live Server" extension in VS Code and right-click on `index.html` to "Open with Live Server".

## Customization

### Colors and Styling
Edit `styles.css` to customize:
- Color scheme (primary colors, gradients)
- Typography (fonts, sizes)
- Layout spacing and dimensions
- Animation timings

### Content
Edit `index.html` to update:
- Hero section text and messaging
- Feature descriptions
- Code examples
- Download links
- Documentation links

### Functionality
Edit `script.js` to modify:
- Interactive behaviors
- Animation triggers
- Form handling
- Code example switching

## Deployment

### GitHub Pages
1. Push the website files to a GitHub repository
2. Go to repository Settings > Pages
3. Select source branch and folder
4. Your site will be available at `https://username.github.io/repository-name`

### Netlify
1. Drag and drop the website folder to Netlify
2. Or connect your GitHub repository
3. Automatic deployment on every push

### Vercel
1. Install Vercel CLI: `npm i -g vercel`
2. Run `vercel` in the website directory
3. Follow the prompts

## Browser Support

- Chrome 60+
- Firefox 55+
- Safari 12+
- Edge 79+

## Performance

The website is optimized for fast loading:
- Minimal external dependencies
- Optimized CSS and JavaScript
- Compressed images
- Efficient animations

## Contributing

To improve the website:
1. Fork the repository
2. Make your changes
3. Test on different devices and browsers
4. Submit a pull request

## License

Same license as the main Utopia project. 