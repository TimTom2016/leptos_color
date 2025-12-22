use csscolorparser::Color;
/// A struct representing the theme configuration for color picker components.
///
/// `Theme` provides a set of customizable properties to control the appearance
/// of color picker components, including colors, dimensions, and style attributes.
pub struct Theme {
    /// The background color of the color picker.
    background: Color,
    /// The background color of input elements within the color picker.
    input_background: Color,
    /// The primary text color used in the color picker.
    color: Color,
    /// The color used for borders in the color picker.
    border_color: Color,
    /// The border radius applied to elements in the color picker.
    border_radius: String,
    /// The box shadow applied to the color picker container.
    box_shadow: String,
    /// The width of the color picker container.
    width: String,
}

impl Theme {
    /// Creates a new `Theme` instance with default (light) settings.
    ///
    /// # Example
    /// ```
    /// let theme = Theme::default();
    /// ```
    pub fn default() -> Self {
        Self::light()
    }
    /// Creates a new `Theme` instance with light theme settings.
    ///
    /// # Example
    /// ```
    /// let light_theme = Theme::light();
    /// ```
    pub fn light() -> Self {
        Self {
            background: "#fff".parse::<Color>().unwrap(),
            input_background: "#f4f4f4".parse::<Color>().unwrap(),
            color: "#262626".parse::<Color>().unwrap(),
            border_color: "#d4d4d4".parse::<Color>().unwrap(),
            border_radius: "4px".to_string(),
            box_shadow: "0px 8px 16px rgba(0, 0, 0, 0.1)".to_string(),
            width: "280px".to_string(),
        }
    }
    /// Creates a new `Theme` instance with dark theme settings.
    ///
    /// # Example
    /// ```
    /// let dark_theme = Theme::dark();
    /// ```
    pub fn dark() -> Self {
        Self {
            background: "rgba(40, 40, 40, 0.95)".parse::<Color>().unwrap(),
            input_background: "#454545".parse::<Color>().unwrap(),
            color: "#e3e3e3".parse::<Color>().unwrap(),
            border_color: "#575657".parse::<Color>().unwrap(),
            border_radius: "4px".to_string(),
            box_shadow: "0px 8px 16px rgba(0, 0, 0, 0.1)".to_string(),
            width: "280px".to_string(),
        }
    }
    /// Creates a new `Theme` instance with custom settings.
    ///
    /// # Arguments
    ///
    /// * `background` - The background color of the color picker.
    /// * `input_background` - The background color of input elements.
    /// * `color` - The primary text color.
    /// * `border_color` - The color used for borders.
    /// * `border_radius` - The border radius as a CSS-compatible string.
    /// * `box_shadow` - The box shadow as a CSS-compatible string.
    /// * `width` - The width of the color picker as a CSS-compatible string.
    ///
    /// # Example
    /// ```
    /// use csscolorparser::Color;
    ///
    /// let custom_theme = Theme::custom(
    ///     "#ffffff".parse().unwrap(),
    ///     "#f0f0f0".parse().unwrap(),
    ///     "#000000".parse().unwrap(),
    ///     "#cccccc".parse().unwrap(),
    ///     "8px".to_string(),
    ///     "0 2px 4px rgba(0,0,0,0.1)".to_string(),
    ///     "300px".to_string(),
    /// );
    /// ```
    pub fn custom(
        background: Color,
        input_background: Color,
        color: Color,
        border_color: Color,
        border_radius: String,
        box_shadow: String,
        width: String,
    ) -> Self {
        Self {
            background,
            input_background: input_background,
            color,
            border_color: border_color,
            border_radius: border_radius,
            box_shadow: box_shadow,
            width,
        }
    }

    pub fn background(&mut self, background: Color) -> &mut Self {
        self.background = background;
        self
    }

    pub fn input_background(&mut self, input_background: Color) -> &mut Self {
        self.input_background = input_background;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn border_color(&mut self, border_color: Color) -> &mut Self {
        self.border_color = border_color;
        self
    }

    pub fn border_radius(&mut self, border_radius: String) -> &mut Self {
        self.border_radius = border_radius;
        self
    }

    pub fn box_shadow(&mut self, box_shadow: String) -> &mut Self {
        self.box_shadow = box_shadow;
        self
    }

    pub fn width(&mut self, width: String) -> &mut Self {
        self.width = width;
        self
    }

    /// Converts the theme settings to a CSS-compatible string.
    ///
    /// This method generates CSS custom properties (variables) that can be
    /// used to apply the theme to color picker components.
    ///
    /// # Example
    /// ```
    /// let theme = Theme::default();
    /// let css = theme.to_style();
    /// println!("CSS variables: {}", css);
    /// ```
    pub fn to_style(&self) -> String {
        format!(
            "--lpc-background: {}; \
                 --lpc-input-background: {}; \
                 --lpc-color: {}; \
                 --lpc-border-color: {}; \
                 --lpc-border-radius: {}; \
                 --lpc-box-shadow: {}; \
                 --lpc-width: {};",
            self.background.to_css_hex(),
            self.input_background.to_css_hex(),
            self.color.to_css_hex(),
            self.border_color.to_css_hex(),
            self.border_radius,
            self.box_shadow,
            self.width
        )
    }
}
