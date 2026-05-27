---
name: Dark Glass
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#393939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1c1b1b'
  surface-container: '#201f1f'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353534'
  on-surface: '#e5e2e1'
  on-surface-variant: '#d9c1bd'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#313030'
  outline: '#a18c88'
  outline-variant: '#544340'
  surface-tint: '#ffb4a7'
  primary: '#ffb4a7'
  on-primary: '#571d14'
  primary-container: '#e28b7c'
  on-primary-container: '#62251c'
  inverse-primary: '#914a3e'
  secondary: '#c8c6c5'
  on-secondary: '#303030'
  secondary-container: '#474746'
  on-secondary-container: '#b6b5b4'
  tertiary: '#81d6bf'
  on-tertiary: '#00382d'
  tertiary-container: '#5cb19b'
  on-tertiary-container: '#004135'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#ffdad4'
  primary-fixed-dim: '#ffb4a7'
  on-primary-fixed: '#3b0804'
  on-primary-fixed-variant: '#743328'
  secondary-fixed: '#e4e2e1'
  secondary-fixed-dim: '#c8c6c5'
  on-secondary-fixed: '#1b1c1c'
  on-secondary-fixed-variant: '#474746'
  tertiary-fixed: '#9df3db'
  tertiary-fixed-dim: '#81d6bf'
  on-tertiary-fixed: '#002019'
  on-tertiary-fixed-variant: '#005142'
  background: '#131313'
  on-background: '#e5e2e1'
  surface-variant: '#353534'
typography:
  display-lg:
    fontFamily: EB Garamond
    fontSize: 48px
    fontWeight: '500'
    lineHeight: '1.1'
    letterSpacing: -0.02em
  display-lg-mobile:
    fontFamily: EB Garamond
    fontSize: 36px
    fontWeight: '500'
    lineHeight: '1.2'
    letterSpacing: -0.01em
  headline-md:
    fontFamily: EB Garamond
    fontSize: 32px
    fontWeight: '500'
    lineHeight: '1.3'
  headline-sm:
    fontFamily: EB Garamond
    fontSize: 24px
    fontWeight: '500'
    lineHeight: '1.4'
  body-lg:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '400'
    lineHeight: '1.6'
  body-md:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: '1.6'
  body-sm:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: '1.5'
  label-caps:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '600'
    lineHeight: '1.2'
    letterSpacing: 0.08em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 8px
  container-max: 1200px
  gutter: 24px
  margin-mobile: 20px
  margin-desktop: 64px
---

## Brand & Style

The design system is an exercise in atmospheric precision, blending **Glassmorphism** with **Minimalist Editorial** aesthetics. It is designed for high-focus environments where depth and hierarchy are communicated through luminosity and blur rather than flat color blocks.

The personality is sophisticated, calm, and premium. It targets users who appreciate a "zen-like" digital workspace—specifically creative professionals, writers, and organizers. The UI evokes a sense of tactile glass layering over a deep, infinite canvas, using high-contrast serif typography to provide an intellectual, authoritative anchor to the ethereal interface.

## Colors

The palette is anchored by a deep **Ink Charcoal (#121212)** base, providing a near-black foundation that minimizes eye strain and maximizes the "pop" of translucent layers. 

- **Primary Accents:** A muted **Terracotta (#E28B7C)** and dusty **Rose (#D4A5A5)** are used sparingly for calls-to-action and critical indicators, ensuring they feel organic rather than synthetic.
- **Surfaces:** Floating elements use a semi-transparent dark grey with a heavy backdrop-blur.
- **Hierarchy:** Depth is achieved through "Luminance Elevation"—the higher an object sits in the Z-space, the lighter and more translucent its background becomes.

## Typography

This system utilizes a high-contrast typographic pairing to balance tradition and utility.

- **Display & Headlines:** **EB Garamond** provides an editorial, literary feel. It should be used for page titles, section headers, and featured quotes. Tighten letter-spacing on larger sizes to maintain a cohesive visual block.
- **UI & Body:** **Inter** handles all functional data. It is chosen for its exceptional legibility in dark mode and its neutral, systematic character. 
- **Labels:** Small labels use Inter in semi-bold with increased tracking for maximum scannability against dark backgrounds.

## Layout & Spacing

The layout follows a **Fixed Grid** philosophy for desktop to maintain the "editorial" feel, centered with generous margins. 

- **Grid:** A 12-column grid on desktop, 4-column on mobile.
- **Rhythm:** An 8px linear scale governs all padding and margins. 
- **Density:** Elements are given significant breathing room (24px - 32px between logical groups) to prevent the "glass" surfaces from feeling cluttered or muddy. 
- **Reflow:** On mobile, margins shrink to 20px, and large Garamond headlines scale down to prevent awkward word breaks, while Inter body text remains consistent for legibility.

## Elevation & Depth

Depth is the defining characteristic of this design system. It is communicated through three specific layers:

1.  **The Canvas (Level 0):** Hex #121212. Static and flat.
2.  **The Glass (Level 1):** Floating panels with a 20px backdrop-blur and a subtle `rgba(255,255,255, 0.08)` hairline border. These panels use a `box-shadow` of 0 20px 40px rgba(0,0,0, 0.4).
3.  **The Interaction (Level 2):** Active states or modals. These increase the surface transparency slightly and add a secondary, sharper shadow to simulate physical proximity to the user.

Avoid solid backgrounds for containers. Every container must exhibit some level of translucency to maintain the "Dark Glass" effect.

## Shapes

The shape language is controlled and modern. 

- **Standard Elements:** Buttons and input fields use a **0.5rem (8px)** corner radius, providing a soft but disciplined look.
- **Large Containers:** Cards and modals use **1rem (16px)** to emphasize their physical presence as objects.
- **Interactive Indicators:** Elements like notification pips or status dots remain perfectly circular.

## Components

- **Buttons:** Primary buttons use the Terracotta accent with white or deep charcoal text. Secondary buttons are "Ghost" style: a hairline border with a subtle glass background that intensifies on hover.
- **Input Fields:** Minimalist design with only a bottom hairline border in the default state, turning into a full glass container on focus.
- **Glass Cards:** The primary organizational unit. Must include the 20px backdrop-blur and the 1px white border at 8% opacity.
- **Chips/Tags:** Small, pill-shaped elements with a slightly lighter grey background (rgba(255,255,255,0.1)) to differentiate from the primary background.
- **Lists:** Separated by thin, 1px horizontal lines at 5% opacity. No heavy borders between items.
- **Modals:** Centered overlays with a darker backdrop dimming effect (rgba(0,0,0,0.6)) to focus the eye on the glass container.
