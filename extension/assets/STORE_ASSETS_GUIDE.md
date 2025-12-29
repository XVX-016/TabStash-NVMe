# Chrome Web Store Assets Guide

This document explains what assets are needed for Chrome Web Store submission.

## Required Assets

### 1. Extension Icons

**Required Sizes**:
- `icon16.png` - 16x16 pixels
- `icon32.png` - 32x32 pixels
- `icon48.png` - 48x48 pixels
- `icon128.png` - 128x128 pixels

**Requirements**:
- Format: PNG with transparency
- Style: Professional, recognizable
- Colors: Work on both light and dark backgrounds
- Design: Simple, clear at small sizes
- Related to: Tab management, storage, or NVMe/SSD concept

**Creation Options**:
1. **Design Tools**: Figma, Adobe Illustrator, Inkscape (free)
2. **Online Generators**: 
   - https://www.favicon-generator.org/
   - https://realfavicongenerator.net/
3. **Icon Libraries**: 
   - Use icons from icon libraries (with proper licensing)
   - Search for "tab", "storage", "disk", "SSD" icons
4. **Hire Designer**: For professional branding

**Placeholder Option** (for testing):
- Create simple colored square with "TS" text
- Use generic storage icon
- **Note**: Replace with proper icon before public release

---

### 2. Store Screenshots

**Required**:
- At least **1 screenshot** (440x280 or 920x680)

**Recommended**:
- **Small Tile**: 440x280 pixels
- **Marquee**: 920x680 pixels
- **Promotional**: 1400x560 pixels (optional)

**What to Screenshot**:

1. **Extension Popup (Connected State)**:
   - Shows "Native host connected" status
   - Clean, professional appearance
   - Highlights key features

2. **Extension Popup (Disconnected State)**:
   - Shows installer download links
   - Demonstrates helpful error handling

3. **Chrome with Extension**:
   - Shows extension icon in toolbar
   - Shows extension in action (if applicable)

**Screenshot Tips**:
- Use clean browser window (no personal tabs)
- Highlight key features
- Add text overlays if helpful (optional)
- Ensure high quality (no blur)
- Use consistent styling

**Tools**:
- Windows: Snipping Tool, Windows + Shift + S
- Linux: Screenshot tool, `gnome-screenshot`
- Browser: Chrome DevTools device toolbar for consistent sizing

---

## Asset Checklist

Before Store Submission:

- [ ] `icon16.png` exists and is 16x16
- [ ] `icon32.png` exists and is 32x32
- [ ] `icon48.png` exists and is 48x48
- [ ] `icon128.png` exists and is 128x128
- [ ] All icons are professional and consistent
- [ ] At least 1 screenshot exists (440x280 or 920x680)
- [ ] Screenshot shows extension in use
- [ ] No placeholders in final submission

---

## Quick Start (Placeholder Icons)

If you need placeholder icons immediately for testing:

### Option 1: Simple Text Icon

1. Create 128x128 image
2. Add colored background (e.g., #667eea)
3. Add white "TS" text in center
4. Export as PNG
5. Resize to other sizes (16, 32, 48)

### Option 2: Use Online Generator

1. Go to https://www.favicon-generator.org/
2. Upload a simple image or use text
3. Generate all sizes
4. Download and rename appropriately

### Option 3: Use Icon Library

1. Search for free icons (e.g., Font Awesome, Material Icons)
2. Download SVG or PNG
3. Resize to required sizes
4. Ensure proper licensing

---

## File Structure

```
extension/assets/
├── icon16.png      (16x16)
├── icon32.png      (32x32)
├── icon48.png      (48x48)
├── icon128.png     (128x128)
└── screenshot.png  (440x280 or 920x680)
```

---

## Store Upload

When uploading to Chrome Web Store:

1. **Icons**: Upload all 4 sizes
2. **Screenshots**: Upload at least small tile (440x280)
3. **Marquee**: Optional but recommended (920x680)
4. **Promotional**: Optional (1400x560)

**Note**: Store will automatically resize if needed, but it's better to provide exact sizes.

---

## Resources

- **Icon Design**: https://material.io/design/iconography/
- **Chrome Web Store Assets**: https://developer.chrome.com/docs/webstore/images/
- **Free Icons**: 
  - https://www.flaticon.com/ (with attribution)
  - https://icons8.com/ (with attribution)
  - https://fontawesome.com/ (free tier)

---

## Next Steps

1. Create or obtain icon files
2. Take screenshot of extension popup
3. Verify all assets meet requirements
4. Test assets in store preview
5. Upload to Chrome Web Store

