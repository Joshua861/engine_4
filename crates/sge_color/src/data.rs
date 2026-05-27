use super::*;
impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const ALICE_BLUE: Self = Self::hex(0xF0F8FF);
    pub const ANTIQUE_WHITE: Self = Self::hex(0xFAEBD7);
    pub const AQUA: Self = Self::hex(0x00FFFF);
    pub const AQUAMARINE: Self = Self::hex(0x7FFFD4);
    pub const AZURE: Self = Self::hex(0xF0FFFF);
    pub const BEIGE: Self = Self::hex(0xF5F5DC);
    pub const BISQUE: Self = Self::hex(0xFFE4C4);
    pub const BLACK: Self = Self::hex(0x000000);
    pub const BLANCHED_ALMOND: Self = Self::hex(0xFFEBCD);
    pub const BLUE: Self = Self::hex(0x0000FF);
    pub const BLUE_VIOLET: Self = Self::hex(0x8A2BE2);
    pub const BROWN: Self = Self::hex(0xA52A2A);
    pub const BURLY_WOOD: Self = Self::hex(0xDEB887);
    pub const CADET_BLUE: Self = Self::hex(0x5F9EA0);
    pub const CHARTREUSE: Self = Self::hex(0x7FFF00);
    pub const CHOCOLATE: Self = Self::hex(0xD2691E);
    pub const CORAL: Self = Self::hex(0xFF7F50);
    pub const CORNFLOWER_BLUE: Self = Self::hex(0x6495ED);
    pub const CORNSILK: Self = Self::hex(0xFFF8DC);
    pub const CRIMSON: Self = Self::hex(0xDC143C);
    pub const CYAN: Self = Self::hex(0x00FFFF);
    pub const DARK_BLUE: Self = Self::hex(0x00008B);
    pub const DARK_CYAN: Self = Self::hex(0x008B8B);
    pub const DARK_GOLDEN_ROD: Self = Self::hex(0xB8860B);
    pub const DARK_GRAY: Self = Self::hex(0xA9A9A9);
    pub const DARK_GREEN: Self = Self::hex(0x006400);
    pub const DARK_GREY: Self = Self::hex(0xA9A9A9);
    pub const DARK_KHAKI: Self = Self::hex(0xBDB76B);
    pub const DARK_MAGENTA: Self = Self::hex(0x8B008B);
    pub const DARK_OLIVE_GREEN: Self = Self::hex(0x556B2F);
    pub const DARK_ORANGE: Self = Self::hex(0xFF8C00);
    pub const DARK_ORCHID: Self = Self::hex(0x9932CC);
    pub const DARK_RED: Self = Self::hex(0x8B0000);
    pub const DARK_SALMON: Self = Self::hex(0xE9967A);
    pub const DARK_SEA_GREEN: Self = Self::hex(0x8FBC8F);
    pub const DARK_SLATE_BLUE: Self = Self::hex(0x483D8B);
    pub const DARK_SLATE_GRAY: Self = Self::hex(0x2F4F4F);
    pub const DARK_SLATE_GREY: Self = Self::hex(0x2F4F4F);
    pub const DARK_TURQUOISE: Self = Self::hex(0x00CED1);
    pub const DARK_VIOLET: Self = Self::hex(0x9400D3);
    pub const DEEP_PINK: Self = Self::hex(0xFF1493);
    pub const DEEP_SKY_BLUE: Self = Self::hex(0x00BFFF);
    pub const DIM_GRAY: Self = Self::hex(0x696969);
    pub const DIM_GREY: Self = Self::hex(0x696969);
    pub const DODGER_BLUE: Self = Self::hex(0x1E90FF);
    pub const FIRE_BRICK: Self = Self::hex(0xB22222);
    pub const FLORAL_WHITE: Self = Self::hex(0xFFFAF0);
    pub const FOREST_GREEN: Self = Self::hex(0x228B22);
    pub const FUCHSIA: Self = Self::hex(0xFF00FF);
    pub const GAINSBORO: Self = Self::hex(0xDCDCDC);
    pub const GHOST_WHITE: Self = Self::hex(0xF8F8FF);
    pub const GOLD: Self = Self::hex(0xFFD700);
    pub const GOLDEN_ROD: Self = Self::hex(0xDAA520);
    pub const GRAY: Self = Self::hex(0x808080);
    pub const GREEN: Self = Self::hex(0x008000);
    pub const GREEN_YELLOW: Self = Self::hex(0xADFF2F);
    pub const GREY: Self = Self::hex(0x808080);
    pub const HONEY_DEW: Self = Self::hex(0xF0FFF0);
    pub const HOT_PINK: Self = Self::hex(0xFF69B4);
    pub const INDIAN_RED: Self = Self::hex(0xCD5C5C);
    pub const INDIGO: Self = Self::hex(0x4B0082);
    pub const IVORY: Self = Self::hex(0xFFFFF0);
    pub const KHAKI: Self = Self::hex(0xF0E68C);
    pub const LAVENDER: Self = Self::hex(0xE6E6FA);
    pub const LAVENDER_BLUSH: Self = Self::hex(0xFFF0F5);
    pub const LAWN_GREEN: Self = Self::hex(0x7CFC00);
    pub const LEMON_CHIFFON: Self = Self::hex(0xFFFACD);
    pub const LIGHT_BLUE: Self = Self::hex(0xADD8E6);
    pub const LIGHT_CORAL: Self = Self::hex(0xF08080);
    pub const LIGHT_CYAN: Self = Self::hex(0xE0FFFF);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::hex(0xFAFAD2);
    pub const LIGHT_GRAY: Self = Self::hex(0xD3D3D3);
    pub const LIGHT_GREEN: Self = Self::hex(0x90EE90);
    pub const LIGHT_GREY: Self = Self::hex(0xD3D3D3);
    pub const LIGHT_PINK: Self = Self::hex(0xFFB6C1);
    pub const LIGHT_SALMON: Self = Self::hex(0xFFA07A);
    pub const LIGHT_SEA_GREEN: Self = Self::hex(0x20B2AA);
    pub const LIGHT_SKY_BLUE: Self = Self::hex(0x87CEFA);
    pub const LIGHT_SLATE_GRAY: Self = Self::hex(0x778899);
    pub const LIGHT_SLATE_GREY: Self = Self::hex(0x778899);
    pub const LIGHT_STEEL_BLUE: Self = Self::hex(0xB0C4DE);
    pub const LIGHT_YELLOW: Self = Self::hex(0xFFFFE0);
    pub const LIME: Self = Self::hex(0x00FF00);
    pub const LIME_GREEN: Self = Self::hex(0x32CD32);
    pub const LINEN: Self = Self::hex(0xFAF0E6);
    pub const MAGENTA: Self = Self::hex(0xFF00FF);
    pub const MAROON: Self = Self::hex(0x800000);
    pub const MEDIUM_AQUA_MARINE: Self = Self::hex(0x66CDAA);
    pub const MEDIUM_BLUE: Self = Self::hex(0x0000CD);
    pub const MEDIUM_ORCHID: Self = Self::hex(0xBA55D3);
    pub const MEDIUM_PURPLE: Self = Self::hex(0x9370DB);
    pub const MEDIUM_SEA_GREEN: Self = Self::hex(0x3CB371);
    pub const MEDIUM_SLATE_BLUE: Self = Self::hex(0x7B68EE);
    pub const MEDIUM_SPRING_GREEN: Self = Self::hex(0x00FA9A);
    pub const MEDIUM_TURQUOISE: Self = Self::hex(0x48D1CC);
    pub const MEDIUM_VIOLET_RED: Self = Self::hex(0xC71585);
    pub const MIDNIGHT_BLUE: Self = Self::hex(0x191970);
    pub const MINT_CREAM: Self = Self::hex(0xF5FFFA);
    pub const MISTY_ROSE: Self = Self::hex(0xFFE4E1);
    pub const MOCCASIN: Self = Self::hex(0xFFE4B5);
    pub const NAVAJO_WHITE: Self = Self::hex(0xFFDEAD);
    pub const NAVY: Self = Self::hex(0x000080);
    pub const OLD_LACE: Self = Self::hex(0xFDF5E6);
    pub const OLIVE: Self = Self::hex(0x808000);
    pub const OLIVE_DRAB: Self = Self::hex(0x6B8E23);
    pub const ORANGE: Self = Self::hex(0xFFA500);
    pub const ORANGE_RED: Self = Self::hex(0xFF4500);
    pub const ORCHID: Self = Self::hex(0xDA70D6);
    pub const PALE_GOLDEN_ROD: Self = Self::hex(0xEEE8AA);
    pub const PALE_GREEN: Self = Self::hex(0x98FB98);
    pub const PALE_TURQUOISE: Self = Self::hex(0xAFEEEE);
    pub const PALE_VIOLET_RED: Self = Self::hex(0xDB7093);
    pub const PAPAYA_WHIP: Self = Self::hex(0xFFEFD5);
    pub const PEACH_PUFF: Self = Self::hex(0xFFDAB9);
    pub const PERU: Self = Self::hex(0xCD853F);
    pub const PINK: Self = Self::hex(0xFFC0CB);
    pub const PLUM: Self = Self::hex(0xDDA0DD);
    pub const POWDER_BLUE: Self = Self::hex(0xB0E0E6);
    pub const PURPLE: Self = Self::hex(0x800080);
    pub const REBECCA_PURPLE: Self = Self::hex(0x663399);
    pub const RED: Self = Self::hex(0xFF0000);
    pub const ROSY_BROWN: Self = Self::hex(0xBC8F8F);
    pub const ROYAL_BLUE: Self = Self::hex(0x4169E1);
    pub const SADDLE_BROWN: Self = Self::hex(0x8B4513);
    pub const SALMON: Self = Self::hex(0xFA8072);
    pub const SANDY_BROWN: Self = Self::hex(0xF4A460);
    pub const SEA_GREEN: Self = Self::hex(0x2E8B57);
    pub const SEA_SHELL: Self = Self::hex(0xFFF5EE);
    pub const SIENNA: Self = Self::hex(0xA0522D);
    pub const SILVER: Self = Self::hex(0xC0C0C0);
    pub const SKY_BLUE: Self = Self::hex(0x87CEEB);
    pub const SLATE_BLUE: Self = Self::hex(0x6A5ACD);
    pub const SLATE_GRAY: Self = Self::hex(0x708090);
    pub const SLATE_GREY: Self = Self::hex(0x708090);
    pub const SNOW: Self = Self::hex(0xFFFAFA);
    pub const SPRING_GREEN: Self = Self::hex(0x00FF7F);
    pub const STEEL_BLUE: Self = Self::hex(0x4682B4);
    pub const TAN: Self = Self::hex(0xD2B48C);
    pub const TEAL: Self = Self::hex(0x008080);
    pub const THISTLE: Self = Self::hex(0xD8BFD8);
    pub const TOMATO: Self = Self::hex(0xFF6347);
    pub const TURQUOISE: Self = Self::hex(0x40E0D0);
    pub const VIOLET: Self = Self::hex(0xEE82EE);
    pub const WHEAT: Self = Self::hex(0xF5DEB3);
    pub const WHITE: Self = Self::hex(0xFFFFFF);
    pub const WHITE_SMOKE: Self = Self::hex(0xF5F5F5);
    pub const YELLOW: Self = Self::hex(0xFFFF00);
    pub const YELLOW_GREEN: Self = Self::hex(0x9ACD32);
    pub const AMBER_50: Self = Self::new(1f32, 0.9845062f32, 0.919994f32);
    pub const AMBER_100: Self = Self::new(0.99700004f32, 0.9530825f32, 0.7775238f32);
    pub const AMBER_200: Self = Self::new(0.99572986f32, 0.90120494f32, 0.5221891f32);
    pub const AMBER_300: Self = Self::new(1f32, 0.8238498f32, 0.187605f32);
    pub const AMBER_400: Self = Self::new(1f32, 0.7272557f32, 0f32);
    pub const AMBER_500: Self = Self::new(0.9942712f32, 0.60209274f32, 0f32);
    pub const AMBER_600: Self = Self::new(0.88376904f32, 0.44335085f32, 0f32);
    pub const AMBER_700: Self = Self::new(0.73178875f32, 0.30102336f32, 0f32);
    pub const AMBER_800: Self = Self::new(0.5903065f32, 0.23375675f32, 0f32);
    pub const AMBER_900: Self = Self::new(0.48044634f32, 0.1997652f32, 0.02483311f32);
    pub const AMBER_950: Self = Self::new(0.2740671f32, 0.098492675f32, 0.004666818f32);
    pub const BLUE_50: Self = Self::new(0.9369396f32, 0.96403474f32, 0.99887186f32);
    pub const BLUE_100: Self = Self::new(0.85824066f32, 0.91782725f32, 0.9972705f32);
    pub const BLUE_200: Self = Self::new(0.7450927f32, 0.85866386f32, 1f32);
    pub const BLUE_300: Self = Self::new(0.5565923f32, 0.7731644f32, 1f32);
    pub const BLUE_400: Self = Self::new(0.31579885f32, 0.63559055f32, 1f32);
    pub const BLUE_500: Self = Self::new(0.16933292f32, 0.49804944f32, 1f32);
    pub const BLUE_600: Self = Self::new(0.08361862f32, 0.36441717f32, 0.9863431f32);
    pub const BLUE_700: Self = Self::new(0.07790815f32, 0.27913508f32, 0.90177447f32);
    pub const BLUE_800: Self = Self::new(0.09978149f32, 0.2337701f32, 0.72292244f32);
    pub const BLUE_900: Self = Self::new(0.10905608f32, 0.22219253f32, 0.55751354f32);
    pub const BLUE_950: Self = Self::new(0.087262414f32, 0.14313233f32, 0.33569205f32);
    pub const CYAN_50: Self = Self::new(0.92500407f32, 0.9960224f32, 0.9999692f32);
    pub const CYAN_100: Self = Self::new(0.80927205f32, 0.9805905f32, 0.99653596f32);
    pub const CYAN_200: Self = Self::new(0.6366474f32, 0.95542425f32, 0.99215513f32);
    pub const CYAN_300: Self = Self::new(0.3266672f32, 0.9169946f32, 0.9913581f32);
    pub const CYAN_400: Self = Self::new(0f32, 0.827362f32, 0.9509646f32);
    pub const CYAN_500: Self = Self::new(0f32, 0.72187245f32, 0.85733795f32);
    pub const CYAN_600: Self = Self::new(0f32, 0.5741973f32, 0.72257245f32);
    pub const CYAN_700: Self = Self::new(0f32, 0.459037f32, 0.5832521f32);
    pub const CYAN_800: Self = Self::new(0f32, 0.3711744f32, 0.47124785f32);
    pub const CYAN_900: Self = Self::new(0.06372786f32, 0.30670863f32, 0.39374155f32);
    pub const CYAN_950: Self = Self::new(0.021188287f32, 0.20064706f32, 0.26983213f32);
    pub const EMERALD_50: Self = Self::new(0.92432135f32, 0.9920315f32, 0.9601801f32);
    pub const EMERALD_100: Self = Self::new(0.81592196f32, 0.9808684f32, 0.8965165f32);
    pub const EMERALD_200: Self = Self::new(0.6444911f32, 0.95581216f32, 0.813324f32);
    pub const EMERALD_300: Self = Self::new(0.3698527f32, 0.9150905f32, 0.70876735f32);
    pub const EMERALD_400: Self = Self::new(0f32, 0.8325921f32, 0.5723413f32);
    pub const EMERALD_500: Self = Self::new(0f32, 0.738789f32, 0.48928064f32);
    pub const EMERALD_600: Self = Self::new(0f32, 0.5989388f32, 0.39898357f32);
    pub const EMERALD_700: Self = Self::new(0f32, 0.4782195f32, 0.33354667f32);
    pub const EMERALD_800: Self = Self::new(0f32, 0.37772787f32, 0.27057654f32);
    pub const EMERALD_900: Self = Self::new(0f32, 0.30810386f32, 0.2298271f32);
    pub const EMERALD_950: Self = Self::new(0f32, 0.17364803f32, 0.13278398f32);
    pub const FUCHSIA_50: Self = Self::new(0.9914102f32, 0.95676965f32, 0.9991073f32);
    pub const FUCHSIA_100: Self = Self::new(0.9815509f32, 0.9089314f32, 1f32);
    pub const FUCHSIA_200: Self = Self::new(0.963965f32, 0.81334287f32, 1f32);
    pub const FUCHSIA_300: Self = Self::new(0.9550995f32, 0.6572811f32, 1f32);
    pub const FUCHSIA_400: Self = Self::new(0.92993844f32, 0.4175663f32, 1f32);
    pub const FUCHSIA_500: Self = Self::new(0.8836879f32, 0.1657466f32, 0.9845014f32);
    pub const FUCHSIA_600: Self = Self::new(0.78360623f32, 0f32, 0.87154114f32);
    pub const FUCHSIA_700: Self = Self::new(0.65847033f32, 0f32, 0.71795124f32);
    pub const FUCHSIA_800: Self = Self::new(
        0.54139715f32,
        0.0051711677f32,
        0.5817819f32,
    );
    pub const FUCHSIA_900: Self = Self::new(0.44791514f32, 0.07534375f32, 0.46959668f32);
    pub const FUCHSIA_950: Self = Self::new(0.2938792f32, 0.0014666162f32, 0.3106184f32);
    pub const GRAY_50: Self = Self::new(0.9763947f32, 0.9809919f32, 0.98558956f32);
    pub const GRAY_100: Self = Self::new(0.9528385f32, 0.95693165f32, 0.96511835f32);
    pub const GRAY_200: Self = Self::new(0.898343f32, 0.90643835f32, 0.92262846f32);
    pub const GRAY_300: Self = Self::new(0.81901777f32, 0.83581483f32, 0.86100876f32);
    pub const GRAY_400: Self = Self::new(0.60000837f32, 0.6313952f32, 0.68522245f32);
    pub const GRAY_500: Self = Self::new(0.41545448f32, 0.44709092f32, 0.5104968f32);
    pub const GRAY_600: Self = Self::new(0.2888335f32, 0.33354583f32, 0.39611638f32);
    pub const GRAY_700: Self = Self::new(0.2115323f32, 0.2550296f32, 0.32470518f32);
    pub const GRAY_800: Self = Self::new(0.11697344f32, 0.16075432f32, 0.22201544f32);
    pub const GRAY_900: Self = Self::new(0.06452563f32, 0.0937416f32, 0.15678442f32);
    pub const GRAY_950: Self = Self::new(0.011584715f32, 0.027607335f32, 0.07187579f32);
    pub const GREEN_50: Self = Self::new(0.9413933f32, 0.992202f32, 0.9570245f32);
    pub const GREEN_100: Self = Self::new(0.86087954f32, 0.988211f32, 0.9046968f32);
    pub const GREEN_200: Self = Self::new(0.72507924f32, 0.9712606f32, 0.8118363f32);
    pub const GREEN_300: Self = Self::new(0.4814555f32, 0.9463487f32, 0.6570022f32);
    pub const GREEN_400: Self = Self::new(0.019873573f32, 0.87531215f32, 0.4485097f32);
    pub const GREEN_500: Self = Self::new(0f32, 0.7871557f32, 0.3156218f32);
    pub const GREEN_600: Self = Self::new(0f32, 0.65130234f32, 0.24199274f32);
    pub const GREEN_700: Self = Self::new(0f32, 0.5099976f32, 0.20980966f32);
    pub const GREEN_800: Self = Self::new(0.0057946057f32, 0.40147665f32, 0.18858832f32);
    pub const GREEN_900: Self = Self::new(0.05112557f32, 0.32868874f32, 0.1701726f32);
    pub const GREEN_950: Self = Self::new(0.011995711f32, 0.18100154f32, 0.08350351f32);
    pub const INDIGO_50: Self = Self::new(0.9333947f32, 0.94913846f32, 1f32);
    pub const INDIGO_100: Self = Self::new(0.87799627f32, 0.9058982f32, 1f32);
    pub const INDIGO_200: Self = Self::new(0.7783514f32, 0.8231922f32, 1f32);
    pub const INDIGO_300: Self = Self::new(0.6391279f32, 0.7021305f32, 1f32);
    pub const INDIGO_400: Self = Self::new(0.48784697f32, 0.5265173f32, 1f32);
    pub const INDIGO_500: Self = Self::new(0.38217616f32, 0.37188828f32, 1f32);
    pub const INDIGO_600: Self = Self::new(0.31088555f32, 0.22440422f32, 0.9662612f32);
    pub const INDIGO_700: Self = Self::new(0.26435313f32, 0.17659259f32, 0.8448339f32);
    pub const INDIGO_800: Self = Self::new(0.21542707f32, 0.16322336f32, 0.67370737f32);
    pub const INDIGO_900: Self = Self::new(0.19164845f32, 0.17264518f32, 0.5226693f32);
    pub const INDIGO_950: Self = Self::new(0.11713376f32, 0.102630235f32, 0.3006191f32);
    pub const LIME_50: Self = Self::new(0.9690419f32, 0.9965202f32, 0.906231f32);
    pub const LIME_100: Self = Self::new(0.925076f32, 0.9888007f32, 0.79337287f32);
    pub const LIME_200: Self = Self::new(0.8477642f32, 0.9783048f32, 0.59990084f32);
    pub const LIME_300: Self = Self::new(0.732937f32, 0.95515925f32, 0.31780392f32);
    pub const LIME_400: Self = Self::new(0.6024432f32, 0.90167123f32, 0f32);
    pub const LIME_500: Self = Self::new(0.48729318f32, 0.80991745f32, 0f32);
    pub const LIME_600: Self = Self::new(0.36872023f32, 0.6474891f32, 0f32);
    pub const LIME_700: Self = Self::new(0.28480813f32, 0.4915021f32, 0f32);
    pub const LIME_800: Self = Self::new(0.23722547f32, 0.3880722f32, 0f32);
    pub const LIME_900: Self = Self::new(0.20701793f32, 0.32722387f32, 0.055236846f32);
    pub const LIME_950: Self = Self::new(0.09908623f32, 0.18128958f32, 0.010939819f32);
    pub const NEUTRAL_50: Self = Self::new(0.980256f32, 0.980256f32, 0.9802559f32);
    pub const NEUTRAL_100: Self = Self::new(0.96058697f32, 0.96058697f32, 0.96058697f32);
    pub const NEUTRAL_200: Self = Self::new(0.8981607f32, 0.8981607f32, 0.8981606f32);
    pub const NEUTRAL_300: Self = Self::new(0.8314444f32, 0.8314444f32, 0.8314444f32);
    pub const NEUTRAL_400: Self = Self::new(0.63016325f32, 0.63016325f32, 0.6301632f32);
    pub const NEUTRAL_500: Self = Self::new(0.4515192f32, 0.45151925f32, 0.4515192f32);
    pub const NEUTRAL_600: Self = Self::new(0.3219929f32, 0.32199287f32, 0.32199284f32);
    pub const NEUTRAL_700: Self = Self::new(0.2504709f32, 0.2504709f32, 0.2504709f32);
    pub const NEUTRAL_800: Self = Self::new(0.14938208f32, 0.14938208f32, 0.14938208f32);
    pub const NEUTRAL_900: Self = Self::new(0.09052741f32, 0.09052741f32, 0.09052741f32);
    pub const NEUTRAL_950: Self = Self::new(
        0.039388236f32,
        0.039388232f32,
        0.039388232f32,
    );
    pub const ORANGE_50: Self = Self::new(1f32, 0.9690531f32, 0.9292628f32);
    pub const ORANGE_100: Self = Self::new(1f32, 0.9288763f32, 0.83258474f32);
    pub const ORANGE_200: Self = Self::new(1f32, 0.8411153f32, 0.6568332f32);
    pub const ORANGE_300: Self = Self::new(1f32, 0.7222757f32, 0.41392696f32);
    pub const ORANGE_400: Self = Self::new(1f32, 0.5370229f32, 0.013841249f32);
    pub const ORANGE_500: Self = Self::new(1f32, 0.41073406f32, 0f32);
    pub const ORANGE_600: Self = Self::new(0.9607056f32, 0.28819042f32, 0f32);
    pub const ORANGE_700: Self = Self::new(0.7918816f32, 0.20727006f32, 0f32);
    pub const ORANGE_800: Self = Self::new(0.62449825f32, 0.17663315f32, 0f32);
    pub const ORANGE_900: Self = Self::new(0.4951185f32, 0.1654189f32, 0.04565227f32);
    pub const ORANGE_950: Self = Self::new(0.26643434f32, 0.07441101f32, 0.02190534f32);
    pub const PINK_50: Self = Self::new(0.99131745f32, 0.948618f32, 0.97190577f32);
    pub const PINK_100: Self = Self::new(0.9886685f32, 0.9052761f32, 0.9529446f32);
    pub const PINK_200: Self = Self::new(0.98767453f32, 0.8090492f32, 0.91072816f32);
    pub const PINK_300: Self = Self::new(0.99408346f32, 0.6464799f32, 0.837142f32);
    pub const PINK_400: Self = Self::new(0.985609f32, 0.3920411f32, 0.7138622f32);
    pub const PINK_500: Self = Self::new(0.9658055f32, 0.19813833f32, 0.6043242f32);
    pub const PINK_600: Self = Self::new(0.90133613f32, 0f32, 0.46341002f32);
    pub const PINK_700: Self = Self::new(0.7777568f32, 0f32, 0.35892293f32);
    pub const PINK_800: Self = Self::new(0.6386032f32, 0f32, 0.29804382f32);
    pub const PINK_900: Self = Self::new(0.5251818f32, 0.06328261f32, 0.26133388f32);
    pub const PINK_950: Self = Self::new(0.31800175f32, 0.015083732f32, 0.13981923f32);
    pub const PURPLE_50: Self = Self::new(0.98043984f32, 0.96110874f32, 0.99977463f32);
    pub const PURPLE_100: Self = Self::new(0.95256f32, 0.9090818f32, 0.9999773f32);
    pub const PURPLE_200: Self = Self::new(0.91501385f32, 0.83308727f32, 1f32);
    pub const PURPLE_300: Self = Self::new(0.8544052f32, 0.6977941f32, 1f32);
    pub const PURPLE_400: Self = Self::new(0.75956774f32, 0.47979993f32, 1f32);
    pub const PURPLE_500: Self = Self::new(0.6779095f32, 0.27594215f32, 1f32);
    pub const PURPLE_600: Self = Self::new(0.5968391f32, 0.061731234f32, 0.9813575f32);
    pub const PURPLE_700: Self = Self::new(0.5099056f32, 0f32, 0.85725754f32);
    pub const PURPLE_800: Self = Self::new(0.42999342f32, 0.066638775f32, 0.6907452f32);
    pub const PURPLE_900: Self = Self::new(0.35072196f32, 0.08730055f32, 0.5451928f32);
    pub const PURPLE_950: Self = Self::new(0.23463124f32, 0.01178017f32, 0.40136918f32);
    pub const RED_50: Self = Self::new(0.996841f32, 0.9495854f32, 0.9495855f32);
    pub const RED_100: Self = Self::new(0.9993008f32, 0.88564295f32, 0.88567674f32);
    pub const RED_200: Self = Self::new(1f32, 0.7898432f32, 0.7900283f32);
    pub const RED_300: Self = Self::new(1f32, 0.6346797f32, 0.6363714f32);
    pub const RED_400: Self = Self::new(1f32, 0.3911524f32, 0.40385672f32);
    pub const RED_500: Self = Self::new(0.9826613f32, 0.17179745f32, 0.21307f32);
    pub const RED_600: Self = Self::new(0.9064574f32, 0f32, 0.042214498f32);
    pub const RED_700: Self = Self::new(0.7568845f32, 0f32, 0.028754465f32);
    pub const RED_800: Self = Self::new(0.6222081f32, 0.028790684f32, 0.06893805f32);
    pub const RED_900: Self = Self::new(0.5091704f32, 0.09232888f32, 0.10068693f32);
    pub const RED_950: Self = Self::new(0.27539662f32, 0.031593174f32, 0.034497585f32);
    pub const ROSE_50: Self = Self::new(0.99902314f32, 0.9447284f32, 0.9486036f32);
    pub const ROSE_100: Self = Self::new(1f32, 0.89342016f32, 0.90132856f32);
    pub const ROSE_200: Self = Self::new(1f32, 0.80110735f32, 0.8256556f32);
    pub const ROSE_300: Self = Self::new(1f32, 0.6299287f32, 0.679516f32);
    pub const ROSE_400: Self = Self::new(1f32, 0.3888606f32, 0.49433142f32);
    pub const ROSE_500: Self = Self::new(1f32, 0.12461511f32, 0.33900315f32);
    pub const ROSE_600: Self = Self::new(0.9273909f32, 0f32, 0.2487092f32);
    pub const ROSE_700: Self = Self::new(0.7785064f32, 0f32, 0.21069628f32);
    pub const ROSE_800: Self = Self::new(0.64669096f32, 0f32, 0.2116679f32);
    pub const ROSE_900: Self = Self::new(0.5445029f32, 0.030038353f32, 0.21069893f32);
    pub const ROSE_950: Self = Self::new(0.30314696f32, 0.008819804f32, 0.0958587f32);
    pub const SKY_50: Self = Self::new(0.939837f32, 0.97659236f32, 1f32);
    pub const SKY_100: Self = Self::new(0.8755858f32, 0.94893783f32, 0.9978108f32);
    pub const SKY_200: Self = Self::new(0.721852f32, 0.90269744f32, 0.9970121f32);
    pub const SKY_300: Self = Self::new(0.45321792f32, 0.83166486f32, 1f32);
    pub const SKY_400: Self = Self::new(0f32, 0.73641187f32, 1f32);
    pub const SKY_500: Self = Self::new(0f32, 0.6497639f32, 0.9571311f32);
    pub const SKY_600: Self = Self::new(0f32, 0.5181665f32, 0.8197996f32);
    pub const SKY_700: Self = Self::new(0f32, 0.41163486f32, 0.6602958f32);
    pub const SKY_800: Self = Self::new(0f32, 0.34901708f32, 0.5398578f32);
    pub const SKY_900: Self = Self::new(0.0067535867f32, 0.29027098f32, 0.4411054f32);
    pub const SKY_950: Self = Self::new(0.019814817f32, 0.18393752f32, 0.2905612f32);
    pub const SLATE_50: Self = Self::new(0.9731485f32, 0.98004276f32, 0.98693746f32);
    pub const SLATE_100: Self = Self::new(0.94447476f32, 0.9604954f32, 0.97651523f32);
    pub const SLATE_200: Self = Self::new(0.88595444f32, 0.910196f32, 0.9425161f32);
    pub const SLATE_300: Self = Self::new(0.792348f32, 0.8358278f32, 0.88797235f32);
    pub const SLATE_400: Self = Self::new(0.56484854f32, 0.6316964f32, 0.7252391f32);
    pub const SLATE_500: Self = Self::new(0.38391194f32, 0.45478725f32, 0.5566605f32);
    pub const SLATE_600: Self = Self::new(0.27099612f32, 0.33408153f32, 0.42418867f32);
    pub const SLATE_700: Self = Self::new(0.19349995f32, 0.2552661f32, 0.34348714f32);
    pub const SLATE_800: Self = Self::new(0.112127654f32, 0.16004395f32, 0.2386725f32);
    pub const SLATE_900: Self = Self::new(0.057184257f32, 0.09004248f32, 0.16879597f32);
    pub const SLATE_950: Self = Self::new(
        0.0074293003f32,
        0.023281762f32,
        0.09250506f32,
    );
    pub const STONE_50: Self = Self::new(0.9805333f32, 0.9805334f32, 0.97756034f32);
    pub const STONE_100: Self = Self::new(0.9608632f32, 0.9608634f32, 0.9579018f32);
    pub const STONE_200: Self = Self::new(0.90670997f32, 0.897532f32, 0.8929411f32);
    pub const STONE_300: Self = Self::new(0.84116566f32, 0.8275092f32, 0.81840104f32);
    pub const STONE_400: Self = Self::new(0.65213054f32, 0.6262227f32, 0.60893804f32);
    pub const STONE_500: Self = Self::new(0.472714f32, 0.44200215f32, 0.42003727f32);
    pub const STONE_600: Self = Self::new(0.34265047f32, 0.32467544f32, 0.30218303f32);
    pub const STONE_700: Self = Self::new(0.268488f32, 0.2504235f32, 0.23233125f32);
    pub const STONE_800: Self = Self::new(0.16162395f32, 0.14419484f32, 0.13983768f32);
    pub const STONE_900: Self = Self::new(0.10951208f32, 0.097987704f32, 0.09030699f32);
    pub const STONE_950: Self = Self::new(
        0.046959206f32,
        0.039356116f32,
        0.035544474f32,
    );
    pub const TEAL_50: Self = Self::new(0.94226456f32, 0.99251837f32, 0.9809168f32);
    pub const TEAL_100: Self = Self::new(0.7967793f32, 0.9858462f32, 0.94572645f32);
    pub const TEAL_200: Self = Self::new(0.5870296f32, 0.9674545f32, 0.8945854f32);
    pub const TEAL_300: Self = Self::new(0.2756205f32, 0.9274332f32, 0.8335046f32);
    pub const TEAL_400: Self = Self::new(0f32, 0.8352503f32, 0.7433175f32);
    pub const TEAL_500: Self = Self::new(0f32, 0.73335874f32, 0.65487087f32);
    pub const TEAL_600: Self = Self::new(0f32, 0.5892682f32, 0.5372885f32);
    pub const TEAL_700: Self = Self::new(0f32, 0.46899873f32, 0.43497157f32);
    pub const TEAL_800: Self = Self::new(0f32, 0.37314332f32, 0.35213464f32);
    pub const TEAL_900: Self = Self::new(0.042003244f32, 0.3081708f32, 0.29175594f32);
    pub const TEAL_950: Self = Self::new(0.008121824f32, 0.18475631f32, 0.18082285f32);
    pub const VIOLET_50: Self = Self::new(0.9605927f32, 0.95277923f32, 0.9996634f32);
    pub const VIOLET_100: Self = Self::new(0.929042f32, 0.91300815f32, 0.9970682f32);
    pub const VIOLET_200: Self = Self::new(0.8665415f32, 0.8378735f32, 1f32);
    pub const VIOLET_300: Self = Self::new(0.7700672f32, 0.70393586f32, 1f32);
    pub const VIOLET_400: Self = Self::new(0.6526052f32, 0.5169348f32, 1f32);
    pub const VIOLET_500: Self = Self::new(0.5559266f32, 0.3181635f32, 1f32);
    pub const VIOLET_600: Self = Self::new(0.49991107f32, 0.13370526f32, 0.9959982f32);
    pub const VIOLET_700: Self = Self::new(0.43968585f32, 0.03142322f32, 0.9064205f32);
    pub const VIOLET_800: Self = Self::new(0.36538363f32, 0.055539064f32, 0.75330627f32);
    pub const VIOLET_900: Self = Self::new(0.30247322f32, 0.08911609f32, 0.6037157f32);
    pub const VIOLET_950: Self = Self::new(0.18254682f32, 0.050902277f32, 0.4063526f32);
    pub const YELLOW_50: Self = Self::new(0.99557024f32, 0.9878028f32, 0.91007847f32);
    pub const YELLOW_100: Self = Self::new(0.9969825f32, 0.97678393f32, 0.7597211f32);
    pub const YELLOW_200: Self = Self::new(0.998684f32, 0.94113076f32, 0.52282774f32);
    pub const YELLOW_300: Self = Self::new(1f32, 0.8762569f32, 0.12571174f32);
    pub const YELLOW_400: Self = Self::new(0.99300355f32, 0.7819445f32, 0f32);
    pub const YELLOW_500: Self = Self::new(0.941312f32, 0.69291544f32, 0f32);
    pub const YELLOW_600: Self = Self::new(0.817554f32, 0.5296361f32, 0f32);
    pub const YELLOW_700: Self = Self::new(0.6510634f32, 0.37320963f32, 0f32);
    pub const YELLOW_800: Self = Self::new(0.53590935f32, 0.29271266f32, 0f32);
    pub const YELLOW_900: Self = Self::new(0.45048618f32, 0.24240229f32, 0.040903658f32);
    pub const YELLOW_950: Self = Self::new(0.26186097f32, 0.12400035f32, 0.017420504f32);
    pub const ZINC_50: Self = Self::new(0.980256f32, 0.980256f32, 0.9802559f32);
    pub const ZINC_100: Self = Self::new(0.9563846f32, 0.9563853f32, 0.9593429f32);
    pub const ZINC_200: Self = Self::new(0.8944766f32, 0.894477f32, 0.90615064f32);
    pub const ZINC_300: Self = Self::new(0.83108747f32, 0.8310831f32, 0.8483447f32);
    pub const ZINC_400: Self = Self::new(0.6226135f32, 0.62256145f32, 0.66335964f32);
    pub const ZINC_500: Self = Self::new(0.44299507f32, 0.4429293f32, 0.48380423f32);
    pub const ZINC_600: Self = Self::new(0.32118192f32, 0.3210902f32, 0.3621075f32);
    pub const ZINC_700: Self = Self::new(0.24648485f32, 0.24644712f32, 0.27646488f32);
    pub const ZINC_800: Self = Self::new(0.152897f32, 0.15288675f32, 0.16577026f32);
    pub const ZINC_900: Self = Self::new(0.093796395f32, 0.09379291f32, 0.10583166f32);
    pub const ZINC_950: Self = Self::new(0.035374288f32, 0.035358995f32, 0.044318125f32);
    pub const ALL: [Self; 242usize] = [
        Self::AMBER_50,
        Self::AMBER_100,
        Self::AMBER_200,
        Self::AMBER_300,
        Self::AMBER_400,
        Self::AMBER_500,
        Self::AMBER_600,
        Self::AMBER_700,
        Self::AMBER_800,
        Self::AMBER_900,
        Self::AMBER_950,
        Self::BLUE_50,
        Self::BLUE_100,
        Self::BLUE_200,
        Self::BLUE_300,
        Self::BLUE_400,
        Self::BLUE_500,
        Self::BLUE_600,
        Self::BLUE_700,
        Self::BLUE_800,
        Self::BLUE_900,
        Self::BLUE_950,
        Self::CYAN_50,
        Self::CYAN_100,
        Self::CYAN_200,
        Self::CYAN_300,
        Self::CYAN_400,
        Self::CYAN_500,
        Self::CYAN_600,
        Self::CYAN_700,
        Self::CYAN_800,
        Self::CYAN_900,
        Self::CYAN_950,
        Self::EMERALD_50,
        Self::EMERALD_100,
        Self::EMERALD_200,
        Self::EMERALD_300,
        Self::EMERALD_400,
        Self::EMERALD_500,
        Self::EMERALD_600,
        Self::EMERALD_700,
        Self::EMERALD_800,
        Self::EMERALD_900,
        Self::EMERALD_950,
        Self::FUCHSIA_50,
        Self::FUCHSIA_100,
        Self::FUCHSIA_200,
        Self::FUCHSIA_300,
        Self::FUCHSIA_400,
        Self::FUCHSIA_500,
        Self::FUCHSIA_600,
        Self::FUCHSIA_700,
        Self::FUCHSIA_800,
        Self::FUCHSIA_900,
        Self::FUCHSIA_950,
        Self::GRAY_50,
        Self::GRAY_100,
        Self::GRAY_200,
        Self::GRAY_300,
        Self::GRAY_400,
        Self::GRAY_500,
        Self::GRAY_600,
        Self::GRAY_700,
        Self::GRAY_800,
        Self::GRAY_900,
        Self::GRAY_950,
        Self::GREEN_50,
        Self::GREEN_100,
        Self::GREEN_200,
        Self::GREEN_300,
        Self::GREEN_400,
        Self::GREEN_500,
        Self::GREEN_600,
        Self::GREEN_700,
        Self::GREEN_800,
        Self::GREEN_900,
        Self::GREEN_950,
        Self::INDIGO_50,
        Self::INDIGO_100,
        Self::INDIGO_200,
        Self::INDIGO_300,
        Self::INDIGO_400,
        Self::INDIGO_500,
        Self::INDIGO_600,
        Self::INDIGO_700,
        Self::INDIGO_800,
        Self::INDIGO_900,
        Self::INDIGO_950,
        Self::LIME_50,
        Self::LIME_100,
        Self::LIME_200,
        Self::LIME_300,
        Self::LIME_400,
        Self::LIME_500,
        Self::LIME_600,
        Self::LIME_700,
        Self::LIME_800,
        Self::LIME_900,
        Self::LIME_950,
        Self::NEUTRAL_50,
        Self::NEUTRAL_100,
        Self::NEUTRAL_200,
        Self::NEUTRAL_300,
        Self::NEUTRAL_400,
        Self::NEUTRAL_500,
        Self::NEUTRAL_600,
        Self::NEUTRAL_700,
        Self::NEUTRAL_800,
        Self::NEUTRAL_900,
        Self::NEUTRAL_950,
        Self::ORANGE_50,
        Self::ORANGE_100,
        Self::ORANGE_200,
        Self::ORANGE_300,
        Self::ORANGE_400,
        Self::ORANGE_500,
        Self::ORANGE_600,
        Self::ORANGE_700,
        Self::ORANGE_800,
        Self::ORANGE_900,
        Self::ORANGE_950,
        Self::PINK_50,
        Self::PINK_100,
        Self::PINK_200,
        Self::PINK_300,
        Self::PINK_400,
        Self::PINK_500,
        Self::PINK_600,
        Self::PINK_700,
        Self::PINK_800,
        Self::PINK_900,
        Self::PINK_950,
        Self::PURPLE_50,
        Self::PURPLE_100,
        Self::PURPLE_200,
        Self::PURPLE_300,
        Self::PURPLE_400,
        Self::PURPLE_500,
        Self::PURPLE_600,
        Self::PURPLE_700,
        Self::PURPLE_800,
        Self::PURPLE_900,
        Self::PURPLE_950,
        Self::RED_50,
        Self::RED_100,
        Self::RED_200,
        Self::RED_300,
        Self::RED_400,
        Self::RED_500,
        Self::RED_600,
        Self::RED_700,
        Self::RED_800,
        Self::RED_900,
        Self::RED_950,
        Self::ROSE_50,
        Self::ROSE_100,
        Self::ROSE_200,
        Self::ROSE_300,
        Self::ROSE_400,
        Self::ROSE_500,
        Self::ROSE_600,
        Self::ROSE_700,
        Self::ROSE_800,
        Self::ROSE_900,
        Self::ROSE_950,
        Self::SKY_50,
        Self::SKY_100,
        Self::SKY_200,
        Self::SKY_300,
        Self::SKY_400,
        Self::SKY_500,
        Self::SKY_600,
        Self::SKY_700,
        Self::SKY_800,
        Self::SKY_900,
        Self::SKY_950,
        Self::SLATE_50,
        Self::SLATE_100,
        Self::SLATE_200,
        Self::SLATE_300,
        Self::SLATE_400,
        Self::SLATE_500,
        Self::SLATE_600,
        Self::SLATE_700,
        Self::SLATE_800,
        Self::SLATE_900,
        Self::SLATE_950,
        Self::STONE_50,
        Self::STONE_100,
        Self::STONE_200,
        Self::STONE_300,
        Self::STONE_400,
        Self::STONE_500,
        Self::STONE_600,
        Self::STONE_700,
        Self::STONE_800,
        Self::STONE_900,
        Self::STONE_950,
        Self::TEAL_50,
        Self::TEAL_100,
        Self::TEAL_200,
        Self::TEAL_300,
        Self::TEAL_400,
        Self::TEAL_500,
        Self::TEAL_600,
        Self::TEAL_700,
        Self::TEAL_800,
        Self::TEAL_900,
        Self::TEAL_950,
        Self::VIOLET_50,
        Self::VIOLET_100,
        Self::VIOLET_200,
        Self::VIOLET_300,
        Self::VIOLET_400,
        Self::VIOLET_500,
        Self::VIOLET_600,
        Self::VIOLET_700,
        Self::VIOLET_800,
        Self::VIOLET_900,
        Self::VIOLET_950,
        Self::YELLOW_50,
        Self::YELLOW_100,
        Self::YELLOW_200,
        Self::YELLOW_300,
        Self::YELLOW_400,
        Self::YELLOW_500,
        Self::YELLOW_600,
        Self::YELLOW_700,
        Self::YELLOW_800,
        Self::YELLOW_900,
        Self::YELLOW_950,
        Self::ZINC_50,
        Self::ZINC_100,
        Self::ZINC_200,
        Self::ZINC_300,
        Self::ZINC_400,
        Self::ZINC_500,
        Self::ZINC_600,
        Self::ZINC_700,
        Self::ZINC_800,
        Self::ZINC_900,
        Self::ZINC_950,
    ];
}
impl Palette {
    pub const AMBER: Self = Self {
        v50: Color::AMBER_50,
        v100: Color::AMBER_100,
        v200: Color::AMBER_200,
        v300: Color::AMBER_300,
        v400: Color::AMBER_400,
        v500: Color::AMBER_500,
        v600: Color::AMBER_600,
        v700: Color::AMBER_700,
        v800: Color::AMBER_800,
        v900: Color::AMBER_900,
        v950: Color::AMBER_950,
    };
    pub const BLUE: Self = Self {
        v50: Color::BLUE_50,
        v100: Color::BLUE_100,
        v200: Color::BLUE_200,
        v300: Color::BLUE_300,
        v400: Color::BLUE_400,
        v500: Color::BLUE_500,
        v600: Color::BLUE_600,
        v700: Color::BLUE_700,
        v800: Color::BLUE_800,
        v900: Color::BLUE_900,
        v950: Color::BLUE_950,
    };
    pub const CYAN: Self = Self {
        v50: Color::CYAN_50,
        v100: Color::CYAN_100,
        v200: Color::CYAN_200,
        v300: Color::CYAN_300,
        v400: Color::CYAN_400,
        v500: Color::CYAN_500,
        v600: Color::CYAN_600,
        v700: Color::CYAN_700,
        v800: Color::CYAN_800,
        v900: Color::CYAN_900,
        v950: Color::CYAN_950,
    };
    pub const EMERALD: Self = Self {
        v50: Color::EMERALD_50,
        v100: Color::EMERALD_100,
        v200: Color::EMERALD_200,
        v300: Color::EMERALD_300,
        v400: Color::EMERALD_400,
        v500: Color::EMERALD_500,
        v600: Color::EMERALD_600,
        v700: Color::EMERALD_700,
        v800: Color::EMERALD_800,
        v900: Color::EMERALD_900,
        v950: Color::EMERALD_950,
    };
    pub const FUCHSIA: Self = Self {
        v50: Color::FUCHSIA_50,
        v100: Color::FUCHSIA_100,
        v200: Color::FUCHSIA_200,
        v300: Color::FUCHSIA_300,
        v400: Color::FUCHSIA_400,
        v500: Color::FUCHSIA_500,
        v600: Color::FUCHSIA_600,
        v700: Color::FUCHSIA_700,
        v800: Color::FUCHSIA_800,
        v900: Color::FUCHSIA_900,
        v950: Color::FUCHSIA_950,
    };
    pub const GRAY: Self = Self {
        v50: Color::GRAY_50,
        v100: Color::GRAY_100,
        v200: Color::GRAY_200,
        v300: Color::GRAY_300,
        v400: Color::GRAY_400,
        v500: Color::GRAY_500,
        v600: Color::GRAY_600,
        v700: Color::GRAY_700,
        v800: Color::GRAY_800,
        v900: Color::GRAY_900,
        v950: Color::GRAY_950,
    };
    pub const GREEN: Self = Self {
        v50: Color::GREEN_50,
        v100: Color::GREEN_100,
        v200: Color::GREEN_200,
        v300: Color::GREEN_300,
        v400: Color::GREEN_400,
        v500: Color::GREEN_500,
        v600: Color::GREEN_600,
        v700: Color::GREEN_700,
        v800: Color::GREEN_800,
        v900: Color::GREEN_900,
        v950: Color::GREEN_950,
    };
    pub const INDIGO: Self = Self {
        v50: Color::INDIGO_50,
        v100: Color::INDIGO_100,
        v200: Color::INDIGO_200,
        v300: Color::INDIGO_300,
        v400: Color::INDIGO_400,
        v500: Color::INDIGO_500,
        v600: Color::INDIGO_600,
        v700: Color::INDIGO_700,
        v800: Color::INDIGO_800,
        v900: Color::INDIGO_900,
        v950: Color::INDIGO_950,
    };
    pub const LIME: Self = Self {
        v50: Color::LIME_50,
        v100: Color::LIME_100,
        v200: Color::LIME_200,
        v300: Color::LIME_300,
        v400: Color::LIME_400,
        v500: Color::LIME_500,
        v600: Color::LIME_600,
        v700: Color::LIME_700,
        v800: Color::LIME_800,
        v900: Color::LIME_900,
        v950: Color::LIME_950,
    };
    pub const NEUTRAL: Self = Self {
        v50: Color::NEUTRAL_50,
        v100: Color::NEUTRAL_100,
        v200: Color::NEUTRAL_200,
        v300: Color::NEUTRAL_300,
        v400: Color::NEUTRAL_400,
        v500: Color::NEUTRAL_500,
        v600: Color::NEUTRAL_600,
        v700: Color::NEUTRAL_700,
        v800: Color::NEUTRAL_800,
        v900: Color::NEUTRAL_900,
        v950: Color::NEUTRAL_950,
    };
    pub const ORANGE: Self = Self {
        v50: Color::ORANGE_50,
        v100: Color::ORANGE_100,
        v200: Color::ORANGE_200,
        v300: Color::ORANGE_300,
        v400: Color::ORANGE_400,
        v500: Color::ORANGE_500,
        v600: Color::ORANGE_600,
        v700: Color::ORANGE_700,
        v800: Color::ORANGE_800,
        v900: Color::ORANGE_900,
        v950: Color::ORANGE_950,
    };
    pub const PINK: Self = Self {
        v50: Color::PINK_50,
        v100: Color::PINK_100,
        v200: Color::PINK_200,
        v300: Color::PINK_300,
        v400: Color::PINK_400,
        v500: Color::PINK_500,
        v600: Color::PINK_600,
        v700: Color::PINK_700,
        v800: Color::PINK_800,
        v900: Color::PINK_900,
        v950: Color::PINK_950,
    };
    pub const PURPLE: Self = Self {
        v50: Color::PURPLE_50,
        v100: Color::PURPLE_100,
        v200: Color::PURPLE_200,
        v300: Color::PURPLE_300,
        v400: Color::PURPLE_400,
        v500: Color::PURPLE_500,
        v600: Color::PURPLE_600,
        v700: Color::PURPLE_700,
        v800: Color::PURPLE_800,
        v900: Color::PURPLE_900,
        v950: Color::PURPLE_950,
    };
    pub const RED: Self = Self {
        v50: Color::RED_50,
        v100: Color::RED_100,
        v200: Color::RED_200,
        v300: Color::RED_300,
        v400: Color::RED_400,
        v500: Color::RED_500,
        v600: Color::RED_600,
        v700: Color::RED_700,
        v800: Color::RED_800,
        v900: Color::RED_900,
        v950: Color::RED_950,
    };
    pub const ROSE: Self = Self {
        v50: Color::ROSE_50,
        v100: Color::ROSE_100,
        v200: Color::ROSE_200,
        v300: Color::ROSE_300,
        v400: Color::ROSE_400,
        v500: Color::ROSE_500,
        v600: Color::ROSE_600,
        v700: Color::ROSE_700,
        v800: Color::ROSE_800,
        v900: Color::ROSE_900,
        v950: Color::ROSE_950,
    };
    pub const SKY: Self = Self {
        v50: Color::SKY_50,
        v100: Color::SKY_100,
        v200: Color::SKY_200,
        v300: Color::SKY_300,
        v400: Color::SKY_400,
        v500: Color::SKY_500,
        v600: Color::SKY_600,
        v700: Color::SKY_700,
        v800: Color::SKY_800,
        v900: Color::SKY_900,
        v950: Color::SKY_950,
    };
    pub const SLATE: Self = Self {
        v50: Color::SLATE_50,
        v100: Color::SLATE_100,
        v200: Color::SLATE_200,
        v300: Color::SLATE_300,
        v400: Color::SLATE_400,
        v500: Color::SLATE_500,
        v600: Color::SLATE_600,
        v700: Color::SLATE_700,
        v800: Color::SLATE_800,
        v900: Color::SLATE_900,
        v950: Color::SLATE_950,
    };
    pub const STONE: Self = Self {
        v50: Color::STONE_50,
        v100: Color::STONE_100,
        v200: Color::STONE_200,
        v300: Color::STONE_300,
        v400: Color::STONE_400,
        v500: Color::STONE_500,
        v600: Color::STONE_600,
        v700: Color::STONE_700,
        v800: Color::STONE_800,
        v900: Color::STONE_900,
        v950: Color::STONE_950,
    };
    pub const TEAL: Self = Self {
        v50: Color::TEAL_50,
        v100: Color::TEAL_100,
        v200: Color::TEAL_200,
        v300: Color::TEAL_300,
        v400: Color::TEAL_400,
        v500: Color::TEAL_500,
        v600: Color::TEAL_600,
        v700: Color::TEAL_700,
        v800: Color::TEAL_800,
        v900: Color::TEAL_900,
        v950: Color::TEAL_950,
    };
    pub const VIOLET: Self = Self {
        v50: Color::VIOLET_50,
        v100: Color::VIOLET_100,
        v200: Color::VIOLET_200,
        v300: Color::VIOLET_300,
        v400: Color::VIOLET_400,
        v500: Color::VIOLET_500,
        v600: Color::VIOLET_600,
        v700: Color::VIOLET_700,
        v800: Color::VIOLET_800,
        v900: Color::VIOLET_900,
        v950: Color::VIOLET_950,
    };
    pub const YELLOW: Self = Self {
        v50: Color::YELLOW_50,
        v100: Color::YELLOW_100,
        v200: Color::YELLOW_200,
        v300: Color::YELLOW_300,
        v400: Color::YELLOW_400,
        v500: Color::YELLOW_500,
        v600: Color::YELLOW_600,
        v700: Color::YELLOW_700,
        v800: Color::YELLOW_800,
        v900: Color::YELLOW_900,
        v950: Color::YELLOW_950,
    };
    pub const ZINC: Self = Self {
        v50: Color::ZINC_50,
        v100: Color::ZINC_100,
        v200: Color::ZINC_200,
        v300: Color::ZINC_300,
        v400: Color::ZINC_400,
        v500: Color::ZINC_500,
        v600: Color::ZINC_600,
        v700: Color::ZINC_700,
        v800: Color::ZINC_800,
        v900: Color::ZINC_900,
        v950: Color::ZINC_950,
    };
    pub const PALETTES: [Self; 22usize] = [
        Self::AMBER,
        Self::BLUE,
        Self::CYAN,
        Self::EMERALD,
        Self::FUCHSIA,
        Self::GRAY,
        Self::GREEN,
        Self::INDIGO,
        Self::LIME,
        Self::NEUTRAL,
        Self::ORANGE,
        Self::PINK,
        Self::PURPLE,
        Self::RED,
        Self::ROSE,
        Self::SKY,
        Self::SLATE,
        Self::STONE,
        Self::TEAL,
        Self::VIOLET,
        Self::YELLOW,
        Self::ZINC,
    ];
}
use phf::phf_map;
pub static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {
    "TRANSPARENT" => Color::TRANSPARENT, "ALICEBLUE" => Color::ALICE_BLUE, "ANTIQUEWHITE"
    => Color::ANTIQUE_WHITE, "AQUA" => Color::AQUA, "AQUAMARINE" => Color::AQUAMARINE,
    "AZURE" => Color::AZURE, "BEIGE" => Color::BEIGE, "BISQUE" => Color::BISQUE, "BLACK"
    => Color::BLACK, "BLANCHEDALMOND" => Color::BLANCHED_ALMOND, "BLUE" => Color::BLUE,
    "BLUEVIOLET" => Color::BLUE_VIOLET, "BROWN" => Color::BROWN, "BURLYWOOD" =>
    Color::BURLY_WOOD, "CADETBLUE" => Color::CADET_BLUE, "CHARTREUSE" =>
    Color::CHARTREUSE, "CHOCOLATE" => Color::CHOCOLATE, "CORAL" => Color::CORAL,
    "CORNFLOWERBLUE" => Color::CORNFLOWER_BLUE, "CORNSILK" => Color::CORNSILK, "CRIMSON"
    => Color::CRIMSON, "CYAN" => Color::CYAN, "DARKBLUE" => Color::DARK_BLUE, "DARKCYAN"
    => Color::DARK_CYAN, "DARKGOLDENROD" => Color::DARK_GOLDEN_ROD, "DARKGRAY" =>
    Color::DARK_GRAY, "DARKGREEN" => Color::DARK_GREEN, "DARKGREY" => Color::DARK_GREY,
    "DARKKHAKI" => Color::DARK_KHAKI, "DARKMAGENTA" => Color::DARK_MAGENTA,
    "DARKOLIVEGREEN" => Color::DARK_OLIVE_GREEN, "DARKORANGE" => Color::DARK_ORANGE,
    "DARKORCHID" => Color::DARK_ORCHID, "DARKRED" => Color::DARK_RED, "DARKSALMON" =>
    Color::DARK_SALMON, "DARKSEAGREEN" => Color::DARK_SEA_GREEN, "DARKSLATEBLUE" =>
    Color::DARK_SLATE_BLUE, "DARKSLATEGRAY" => Color::DARK_SLATE_GRAY, "DARKSLATEGREY" =>
    Color::DARK_SLATE_GREY, "DARKTURQUOISE" => Color::DARK_TURQUOISE, "DARKVIOLET" =>
    Color::DARK_VIOLET, "DEEPPINK" => Color::DEEP_PINK, "DEEPSKYBLUE" =>
    Color::DEEP_SKY_BLUE, "DIMGRAY" => Color::DIM_GRAY, "DIMGREY" => Color::DIM_GREY,
    "DODGERBLUE" => Color::DODGER_BLUE, "FIREBRICK" => Color::FIRE_BRICK, "FLORALWHITE"
    => Color::FLORAL_WHITE, "FORESTGREEN" => Color::FOREST_GREEN, "FUCHSIA" =>
    Color::FUCHSIA, "GAINSBORO" => Color::GAINSBORO, "GHOSTWHITE" => Color::GHOST_WHITE,
    "GOLD" => Color::GOLD, "GOLDENROD" => Color::GOLDEN_ROD, "GRAY" => Color::GRAY,
    "GREEN" => Color::GREEN, "GREENYELLOW" => Color::GREEN_YELLOW, "GREY" => Color::GREY,
    "HONEYDEW" => Color::HONEY_DEW, "HOTPINK" => Color::HOT_PINK, "INDIANRED" =>
    Color::INDIAN_RED, "INDIGO" => Color::INDIGO, "IVORY" => Color::IVORY, "KHAKI" =>
    Color::KHAKI, "LAVENDER" => Color::LAVENDER, "LAVENDERBLUSH" =>
    Color::LAVENDER_BLUSH, "LAWNGREEN" => Color::LAWN_GREEN, "LEMONCHIFFON" =>
    Color::LEMON_CHIFFON, "LIGHTBLUE" => Color::LIGHT_BLUE, "LIGHTCORAL" =>
    Color::LIGHT_CORAL, "LIGHTCYAN" => Color::LIGHT_CYAN, "LIGHTGOLDENRODYELLOW" =>
    Color::LIGHT_GOLDEN_ROD_YELLOW, "LIGHTGRAY" => Color::LIGHT_GRAY, "LIGHTGREEN" =>
    Color::LIGHT_GREEN, "LIGHTGREY" => Color::LIGHT_GREY, "LIGHTPINK" =>
    Color::LIGHT_PINK, "LIGHTSALMON" => Color::LIGHT_SALMON, "LIGHTSEAGREEN" =>
    Color::LIGHT_SEA_GREEN, "LIGHTSKYBLUE" => Color::LIGHT_SKY_BLUE, "LIGHTSLATEGRAY" =>
    Color::LIGHT_SLATE_GRAY, "LIGHTSLATEGREY" => Color::LIGHT_SLATE_GREY,
    "LIGHTSTEELBLUE" => Color::LIGHT_STEEL_BLUE, "LIGHTYELLOW" => Color::LIGHT_YELLOW,
    "LIME" => Color::LIME, "LIMEGREEN" => Color::LIME_GREEN, "LINEN" => Color::LINEN,
    "MAGENTA" => Color::MAGENTA, "MAROON" => Color::MAROON, "MEDIUMAQUAMARINE" =>
    Color::MEDIUM_AQUA_MARINE, "MEDIUMBLUE" => Color::MEDIUM_BLUE, "MEDIUMORCHID" =>
    Color::MEDIUM_ORCHID, "MEDIUMPURPLE" => Color::MEDIUM_PURPLE, "MEDIUMSEAGREEN" =>
    Color::MEDIUM_SEA_GREEN, "MEDIUMSLATEBLUE" => Color::MEDIUM_SLATE_BLUE,
    "MEDIUMSPRINGGREEN" => Color::MEDIUM_SPRING_GREEN, "MEDIUMTURQUOISE" =>
    Color::MEDIUM_TURQUOISE, "MEDIUMVIOLETRED" => Color::MEDIUM_VIOLET_RED,
    "MIDNIGHTBLUE" => Color::MIDNIGHT_BLUE, "MINTCREAM" => Color::MINT_CREAM, "MISTYROSE"
    => Color::MISTY_ROSE, "MOCCASIN" => Color::MOCCASIN, "NAVAJOWHITE" =>
    Color::NAVAJO_WHITE, "NAVY" => Color::NAVY, "OLDLACE" => Color::OLD_LACE, "OLIVE" =>
    Color::OLIVE, "OLIVEDRAB" => Color::OLIVE_DRAB, "ORANGE" => Color::ORANGE,
    "ORANGERED" => Color::ORANGE_RED, "ORCHID" => Color::ORCHID, "PALEGOLDENROD" =>
    Color::PALE_GOLDEN_ROD, "PALEGREEN" => Color::PALE_GREEN, "PALETURQUOISE" =>
    Color::PALE_TURQUOISE, "PALEVIOLETRED" => Color::PALE_VIOLET_RED, "PAPAYAWHIP" =>
    Color::PAPAYA_WHIP, "PEACHPUFF" => Color::PEACH_PUFF, "PERU" => Color::PERU, "PINK"
    => Color::PINK, "PLUM" => Color::PLUM, "POWDERBLUE" => Color::POWDER_BLUE, "PURPLE"
    => Color::PURPLE, "REBECCAPURPLE" => Color::REBECCA_PURPLE, "RED" => Color::RED,
    "ROSYBROWN" => Color::ROSY_BROWN, "ROYALBLUE" => Color::ROYAL_BLUE, "SADDLEBROWN" =>
    Color::SADDLE_BROWN, "SALMON" => Color::SALMON, "SANDYBROWN" => Color::SANDY_BROWN,
    "SEAGREEN" => Color::SEA_GREEN, "SEASHELL" => Color::SEA_SHELL, "SIENNA" =>
    Color::SIENNA, "SILVER" => Color::SILVER, "SKYBLUE" => Color::SKY_BLUE, "SLATEBLUE"
    => Color::SLATE_BLUE, "SLATEGRAY" => Color::SLATE_GRAY, "SLATEGREY" =>
    Color::SLATE_GREY, "SNOW" => Color::SNOW, "SPRINGGREEN" => Color::SPRING_GREEN,
    "STEELBLUE" => Color::STEEL_BLUE, "TAN" => Color::TAN, "TEAL" => Color::TEAL,
    "THISTLE" => Color::THISTLE, "TOMATO" => Color::TOMATO, "TURQUOISE" =>
    Color::TURQUOISE, "VIOLET" => Color::VIOLET, "WHEAT" => Color::WHEAT, "WHITE" =>
    Color::WHITE, "WHITESMOKE" => Color::WHITE_SMOKE, "YELLOW" => Color::YELLOW,
    "YELLOWGREEN" => Color::YELLOW_GREEN, "AMBER_50" => Color::AMBER_50, "AMBER0.5" =>
    Color::AMBER_50, "AMBER05" => Color::AMBER_50, "AMBER_100" => Color::AMBER_100,
    "AMBER1" => Color::AMBER_100, "AMBER_200" => Color::AMBER_200, "AMBER2" =>
    Color::AMBER_200, "AMBER_300" => Color::AMBER_300, "AMBER3" => Color::AMBER_300,
    "AMBER_400" => Color::AMBER_400, "AMBER4" => Color::AMBER_400, "AMBER_500" =>
    Color::AMBER_500, "AMBER5" => Color::AMBER_500, "AMBER_600" => Color::AMBER_600,
    "AMBER6" => Color::AMBER_600, "AMBER_700" => Color::AMBER_700, "AMBER7" =>
    Color::AMBER_700, "AMBER_800" => Color::AMBER_800, "AMBER8" => Color::AMBER_800,
    "AMBER_900" => Color::AMBER_900, "AMBER9" => Color::AMBER_900, "AMBER_950" =>
    Color::AMBER_950, "AMBER9.5" => Color::AMBER_950, "AMBER95" => Color::AMBER_950,
    "BLUE_50" => Color::BLUE_50, "BLUE0.5" => Color::BLUE_50, "BLUE05" => Color::BLUE_50,
    "BLUE_100" => Color::BLUE_100, "BLUE1" => Color::BLUE_100, "BLUE_200" =>
    Color::BLUE_200, "BLUE2" => Color::BLUE_200, "BLUE_300" => Color::BLUE_300, "BLUE3"
    => Color::BLUE_300, "BLUE_400" => Color::BLUE_400, "BLUE4" => Color::BLUE_400,
    "BLUE_500" => Color::BLUE_500, "BLUE5" => Color::BLUE_500, "BLUE_600" =>
    Color::BLUE_600, "BLUE6" => Color::BLUE_600, "BLUE_700" => Color::BLUE_700, "BLUE7"
    => Color::BLUE_700, "BLUE_800" => Color::BLUE_800, "BLUE8" => Color::BLUE_800,
    "BLUE_900" => Color::BLUE_900, "BLUE9" => Color::BLUE_900, "BLUE_950" =>
    Color::BLUE_950, "BLUE9.5" => Color::BLUE_950, "BLUE95" => Color::BLUE_950, "CYAN_50"
    => Color::CYAN_50, "CYAN0.5" => Color::CYAN_50, "CYAN05" => Color::CYAN_50,
    "CYAN_100" => Color::CYAN_100, "CYAN1" => Color::CYAN_100, "CYAN_200" =>
    Color::CYAN_200, "CYAN2" => Color::CYAN_200, "CYAN_300" => Color::CYAN_300, "CYAN3"
    => Color::CYAN_300, "CYAN_400" => Color::CYAN_400, "CYAN4" => Color::CYAN_400,
    "CYAN_500" => Color::CYAN_500, "CYAN5" => Color::CYAN_500, "CYAN_600" =>
    Color::CYAN_600, "CYAN6" => Color::CYAN_600, "CYAN_700" => Color::CYAN_700, "CYAN7"
    => Color::CYAN_700, "CYAN_800" => Color::CYAN_800, "CYAN8" => Color::CYAN_800,
    "CYAN_900" => Color::CYAN_900, "CYAN9" => Color::CYAN_900, "CYAN_950" =>
    Color::CYAN_950, "CYAN9.5" => Color::CYAN_950, "CYAN95" => Color::CYAN_950,
    "EMERALD_50" => Color::EMERALD_50, "EMERALD0.5" => Color::EMERALD_50, "EMERALD05" =>
    Color::EMERALD_50, "EMERALD_100" => Color::EMERALD_100, "EMERALD1" =>
    Color::EMERALD_100, "EMERALD_200" => Color::EMERALD_200, "EMERALD2" =>
    Color::EMERALD_200, "EMERALD_300" => Color::EMERALD_300, "EMERALD3" =>
    Color::EMERALD_300, "EMERALD_400" => Color::EMERALD_400, "EMERALD4" =>
    Color::EMERALD_400, "EMERALD_500" => Color::EMERALD_500, "EMERALD5" =>
    Color::EMERALD_500, "EMERALD_600" => Color::EMERALD_600, "EMERALD6" =>
    Color::EMERALD_600, "EMERALD_700" => Color::EMERALD_700, "EMERALD7" =>
    Color::EMERALD_700, "EMERALD_800" => Color::EMERALD_800, "EMERALD8" =>
    Color::EMERALD_800, "EMERALD_900" => Color::EMERALD_900, "EMERALD9" =>
    Color::EMERALD_900, "EMERALD_950" => Color::EMERALD_950, "EMERALD9.5" =>
    Color::EMERALD_950, "EMERALD95" => Color::EMERALD_950, "FUCHSIA_50" =>
    Color::FUCHSIA_50, "FUCHSIA0.5" => Color::FUCHSIA_50, "FUCHSIA05" =>
    Color::FUCHSIA_50, "FUCHSIA_100" => Color::FUCHSIA_100, "FUCHSIA1" =>
    Color::FUCHSIA_100, "FUCHSIA_200" => Color::FUCHSIA_200, "FUCHSIA2" =>
    Color::FUCHSIA_200, "FUCHSIA_300" => Color::FUCHSIA_300, "FUCHSIA3" =>
    Color::FUCHSIA_300, "FUCHSIA_400" => Color::FUCHSIA_400, "FUCHSIA4" =>
    Color::FUCHSIA_400, "FUCHSIA_500" => Color::FUCHSIA_500, "FUCHSIA5" =>
    Color::FUCHSIA_500, "FUCHSIA_600" => Color::FUCHSIA_600, "FUCHSIA6" =>
    Color::FUCHSIA_600, "FUCHSIA_700" => Color::FUCHSIA_700, "FUCHSIA7" =>
    Color::FUCHSIA_700, "FUCHSIA_800" => Color::FUCHSIA_800, "FUCHSIA8" =>
    Color::FUCHSIA_800, "FUCHSIA_900" => Color::FUCHSIA_900, "FUCHSIA9" =>
    Color::FUCHSIA_900, "FUCHSIA_950" => Color::FUCHSIA_950, "FUCHSIA9.5" =>
    Color::FUCHSIA_950, "FUCHSIA95" => Color::FUCHSIA_950, "GRAY_50" => Color::GRAY_50,
    "GRAY0.5" => Color::GRAY_50, "GRAY05" => Color::GRAY_50, "GRAY_100" =>
    Color::GRAY_100, "GRAY1" => Color::GRAY_100, "GRAY_200" => Color::GRAY_200, "GRAY2"
    => Color::GRAY_200, "GRAY_300" => Color::GRAY_300, "GRAY3" => Color::GRAY_300,
    "GRAY_400" => Color::GRAY_400, "GRAY4" => Color::GRAY_400, "GRAY_500" =>
    Color::GRAY_500, "GRAY5" => Color::GRAY_500, "GRAY_600" => Color::GRAY_600, "GRAY6"
    => Color::GRAY_600, "GRAY_700" => Color::GRAY_700, "GRAY7" => Color::GRAY_700,
    "GRAY_800" => Color::GRAY_800, "GRAY8" => Color::GRAY_800, "GRAY_900" =>
    Color::GRAY_900, "GRAY9" => Color::GRAY_900, "GRAY_950" => Color::GRAY_950, "GRAY9.5"
    => Color::GRAY_950, "GRAY95" => Color::GRAY_950, "GREEN_50" => Color::GREEN_50,
    "GREEN0.5" => Color::GREEN_50, "GREEN05" => Color::GREEN_50, "GREEN_100" =>
    Color::GREEN_100, "GREEN1" => Color::GREEN_100, "GREEN_200" => Color::GREEN_200,
    "GREEN2" => Color::GREEN_200, "GREEN_300" => Color::GREEN_300, "GREEN3" =>
    Color::GREEN_300, "GREEN_400" => Color::GREEN_400, "GREEN4" => Color::GREEN_400,
    "GREEN_500" => Color::GREEN_500, "GREEN5" => Color::GREEN_500, "GREEN_600" =>
    Color::GREEN_600, "GREEN6" => Color::GREEN_600, "GREEN_700" => Color::GREEN_700,
    "GREEN7" => Color::GREEN_700, "GREEN_800" => Color::GREEN_800, "GREEN8" =>
    Color::GREEN_800, "GREEN_900" => Color::GREEN_900, "GREEN9" => Color::GREEN_900,
    "GREEN_950" => Color::GREEN_950, "GREEN9.5" => Color::GREEN_950, "GREEN95" =>
    Color::GREEN_950, "INDIGO_50" => Color::INDIGO_50, "INDIGO0.5" => Color::INDIGO_50,
    "INDIGO05" => Color::INDIGO_50, "INDIGO_100" => Color::INDIGO_100, "INDIGO1" =>
    Color::INDIGO_100, "INDIGO_200" => Color::INDIGO_200, "INDIGO2" => Color::INDIGO_200,
    "INDIGO_300" => Color::INDIGO_300, "INDIGO3" => Color::INDIGO_300, "INDIGO_400" =>
    Color::INDIGO_400, "INDIGO4" => Color::INDIGO_400, "INDIGO_500" => Color::INDIGO_500,
    "INDIGO5" => Color::INDIGO_500, "INDIGO_600" => Color::INDIGO_600, "INDIGO6" =>
    Color::INDIGO_600, "INDIGO_700" => Color::INDIGO_700, "INDIGO7" => Color::INDIGO_700,
    "INDIGO_800" => Color::INDIGO_800, "INDIGO8" => Color::INDIGO_800, "INDIGO_900" =>
    Color::INDIGO_900, "INDIGO9" => Color::INDIGO_900, "INDIGO_950" => Color::INDIGO_950,
    "INDIGO9.5" => Color::INDIGO_950, "INDIGO95" => Color::INDIGO_950, "LIME_50" =>
    Color::LIME_50, "LIME0.5" => Color::LIME_50, "LIME05" => Color::LIME_50, "LIME_100"
    => Color::LIME_100, "LIME1" => Color::LIME_100, "LIME_200" => Color::LIME_200,
    "LIME2" => Color::LIME_200, "LIME_300" => Color::LIME_300, "LIME3" =>
    Color::LIME_300, "LIME_400" => Color::LIME_400, "LIME4" => Color::LIME_400,
    "LIME_500" => Color::LIME_500, "LIME5" => Color::LIME_500, "LIME_600" =>
    Color::LIME_600, "LIME6" => Color::LIME_600, "LIME_700" => Color::LIME_700, "LIME7"
    => Color::LIME_700, "LIME_800" => Color::LIME_800, "LIME8" => Color::LIME_800,
    "LIME_900" => Color::LIME_900, "LIME9" => Color::LIME_900, "LIME_950" =>
    Color::LIME_950, "LIME9.5" => Color::LIME_950, "LIME95" => Color::LIME_950,
    "NEUTRAL_50" => Color::NEUTRAL_50, "NEUTRAL0.5" => Color::NEUTRAL_50, "NEUTRAL05" =>
    Color::NEUTRAL_50, "NEUTRAL_100" => Color::NEUTRAL_100, "NEUTRAL1" =>
    Color::NEUTRAL_100, "NEUTRAL_200" => Color::NEUTRAL_200, "NEUTRAL2" =>
    Color::NEUTRAL_200, "NEUTRAL_300" => Color::NEUTRAL_300, "NEUTRAL3" =>
    Color::NEUTRAL_300, "NEUTRAL_400" => Color::NEUTRAL_400, "NEUTRAL4" =>
    Color::NEUTRAL_400, "NEUTRAL_500" => Color::NEUTRAL_500, "NEUTRAL5" =>
    Color::NEUTRAL_500, "NEUTRAL_600" => Color::NEUTRAL_600, "NEUTRAL6" =>
    Color::NEUTRAL_600, "NEUTRAL_700" => Color::NEUTRAL_700, "NEUTRAL7" =>
    Color::NEUTRAL_700, "NEUTRAL_800" => Color::NEUTRAL_800, "NEUTRAL8" =>
    Color::NEUTRAL_800, "NEUTRAL_900" => Color::NEUTRAL_900, "NEUTRAL9" =>
    Color::NEUTRAL_900, "NEUTRAL_950" => Color::NEUTRAL_950, "NEUTRAL9.5" =>
    Color::NEUTRAL_950, "NEUTRAL95" => Color::NEUTRAL_950, "ORANGE_50" =>
    Color::ORANGE_50, "ORANGE0.5" => Color::ORANGE_50, "ORANGE05" => Color::ORANGE_50,
    "ORANGE_100" => Color::ORANGE_100, "ORANGE1" => Color::ORANGE_100, "ORANGE_200" =>
    Color::ORANGE_200, "ORANGE2" => Color::ORANGE_200, "ORANGE_300" => Color::ORANGE_300,
    "ORANGE3" => Color::ORANGE_300, "ORANGE_400" => Color::ORANGE_400, "ORANGE4" =>
    Color::ORANGE_400, "ORANGE_500" => Color::ORANGE_500, "ORANGE5" => Color::ORANGE_500,
    "ORANGE_600" => Color::ORANGE_600, "ORANGE6" => Color::ORANGE_600, "ORANGE_700" =>
    Color::ORANGE_700, "ORANGE7" => Color::ORANGE_700, "ORANGE_800" => Color::ORANGE_800,
    "ORANGE8" => Color::ORANGE_800, "ORANGE_900" => Color::ORANGE_900, "ORANGE9" =>
    Color::ORANGE_900, "ORANGE_950" => Color::ORANGE_950, "ORANGE9.5" =>
    Color::ORANGE_950, "ORANGE95" => Color::ORANGE_950, "PINK_50" => Color::PINK_50,
    "PINK0.5" => Color::PINK_50, "PINK05" => Color::PINK_50, "PINK_100" =>
    Color::PINK_100, "PINK1" => Color::PINK_100, "PINK_200" => Color::PINK_200, "PINK2"
    => Color::PINK_200, "PINK_300" => Color::PINK_300, "PINK3" => Color::PINK_300,
    "PINK_400" => Color::PINK_400, "PINK4" => Color::PINK_400, "PINK_500" =>
    Color::PINK_500, "PINK5" => Color::PINK_500, "PINK_600" => Color::PINK_600, "PINK6"
    => Color::PINK_600, "PINK_700" => Color::PINK_700, "PINK7" => Color::PINK_700,
    "PINK_800" => Color::PINK_800, "PINK8" => Color::PINK_800, "PINK_900" =>
    Color::PINK_900, "PINK9" => Color::PINK_900, "PINK_950" => Color::PINK_950, "PINK9.5"
    => Color::PINK_950, "PINK95" => Color::PINK_950, "PURPLE_50" => Color::PURPLE_50,
    "PURPLE0.5" => Color::PURPLE_50, "PURPLE05" => Color::PURPLE_50, "PURPLE_100" =>
    Color::PURPLE_100, "PURPLE1" => Color::PURPLE_100, "PURPLE_200" => Color::PURPLE_200,
    "PURPLE2" => Color::PURPLE_200, "PURPLE_300" => Color::PURPLE_300, "PURPLE3" =>
    Color::PURPLE_300, "PURPLE_400" => Color::PURPLE_400, "PURPLE4" => Color::PURPLE_400,
    "PURPLE_500" => Color::PURPLE_500, "PURPLE5" => Color::PURPLE_500, "PURPLE_600" =>
    Color::PURPLE_600, "PURPLE6" => Color::PURPLE_600, "PURPLE_700" => Color::PURPLE_700,
    "PURPLE7" => Color::PURPLE_700, "PURPLE_800" => Color::PURPLE_800, "PURPLE8" =>
    Color::PURPLE_800, "PURPLE_900" => Color::PURPLE_900, "PURPLE9" => Color::PURPLE_900,
    "PURPLE_950" => Color::PURPLE_950, "PURPLE9.5" => Color::PURPLE_950, "PURPLE95" =>
    Color::PURPLE_950, "RED_50" => Color::RED_50, "RED0.5" => Color::RED_50, "RED05" =>
    Color::RED_50, "RED_100" => Color::RED_100, "RED1" => Color::RED_100, "RED_200" =>
    Color::RED_200, "RED2" => Color::RED_200, "RED_300" => Color::RED_300, "RED3" =>
    Color::RED_300, "RED_400" => Color::RED_400, "RED4" => Color::RED_400, "RED_500" =>
    Color::RED_500, "RED5" => Color::RED_500, "RED_600" => Color::RED_600, "RED6" =>
    Color::RED_600, "RED_700" => Color::RED_700, "RED7" => Color::RED_700, "RED_800" =>
    Color::RED_800, "RED8" => Color::RED_800, "RED_900" => Color::RED_900, "RED9" =>
    Color::RED_900, "RED_950" => Color::RED_950, "RED9.5" => Color::RED_950, "RED95" =>
    Color::RED_950, "ROSE_50" => Color::ROSE_50, "ROSE0.5" => Color::ROSE_50, "ROSE05" =>
    Color::ROSE_50, "ROSE_100" => Color::ROSE_100, "ROSE1" => Color::ROSE_100, "ROSE_200"
    => Color::ROSE_200, "ROSE2" => Color::ROSE_200, "ROSE_300" => Color::ROSE_300,
    "ROSE3" => Color::ROSE_300, "ROSE_400" => Color::ROSE_400, "ROSE4" =>
    Color::ROSE_400, "ROSE_500" => Color::ROSE_500, "ROSE5" => Color::ROSE_500,
    "ROSE_600" => Color::ROSE_600, "ROSE6" => Color::ROSE_600, "ROSE_700" =>
    Color::ROSE_700, "ROSE7" => Color::ROSE_700, "ROSE_800" => Color::ROSE_800, "ROSE8"
    => Color::ROSE_800, "ROSE_900" => Color::ROSE_900, "ROSE9" => Color::ROSE_900,
    "ROSE_950" => Color::ROSE_950, "ROSE9.5" => Color::ROSE_950, "ROSE95" =>
    Color::ROSE_950, "SKY_50" => Color::SKY_50, "SKY0.5" => Color::SKY_50, "SKY05" =>
    Color::SKY_50, "SKY_100" => Color::SKY_100, "SKY1" => Color::SKY_100, "SKY_200" =>
    Color::SKY_200, "SKY2" => Color::SKY_200, "SKY_300" => Color::SKY_300, "SKY3" =>
    Color::SKY_300, "SKY_400" => Color::SKY_400, "SKY4" => Color::SKY_400, "SKY_500" =>
    Color::SKY_500, "SKY5" => Color::SKY_500, "SKY_600" => Color::SKY_600, "SKY6" =>
    Color::SKY_600, "SKY_700" => Color::SKY_700, "SKY7" => Color::SKY_700, "SKY_800" =>
    Color::SKY_800, "SKY8" => Color::SKY_800, "SKY_900" => Color::SKY_900, "SKY9" =>
    Color::SKY_900, "SKY_950" => Color::SKY_950, "SKY9.5" => Color::SKY_950, "SKY95" =>
    Color::SKY_950, "SLATE_50" => Color::SLATE_50, "SLATE0.5" => Color::SLATE_50,
    "SLATE05" => Color::SLATE_50, "SLATE_100" => Color::SLATE_100, "SLATE1" =>
    Color::SLATE_100, "SLATE_200" => Color::SLATE_200, "SLATE2" => Color::SLATE_200,
    "SLATE_300" => Color::SLATE_300, "SLATE3" => Color::SLATE_300, "SLATE_400" =>
    Color::SLATE_400, "SLATE4" => Color::SLATE_400, "SLATE_500" => Color::SLATE_500,
    "SLATE5" => Color::SLATE_500, "SLATE_600" => Color::SLATE_600, "SLATE6" =>
    Color::SLATE_600, "SLATE_700" => Color::SLATE_700, "SLATE7" => Color::SLATE_700,
    "SLATE_800" => Color::SLATE_800, "SLATE8" => Color::SLATE_800, "SLATE_900" =>
    Color::SLATE_900, "SLATE9" => Color::SLATE_900, "SLATE_950" => Color::SLATE_950,
    "SLATE9.5" => Color::SLATE_950, "SLATE95" => Color::SLATE_950, "STONE_50" =>
    Color::STONE_50, "STONE0.5" => Color::STONE_50, "STONE05" => Color::STONE_50,
    "STONE_100" => Color::STONE_100, "STONE1" => Color::STONE_100, "STONE_200" =>
    Color::STONE_200, "STONE2" => Color::STONE_200, "STONE_300" => Color::STONE_300,
    "STONE3" => Color::STONE_300, "STONE_400" => Color::STONE_400, "STONE4" =>
    Color::STONE_400, "STONE_500" => Color::STONE_500, "STONE5" => Color::STONE_500,
    "STONE_600" => Color::STONE_600, "STONE6" => Color::STONE_600, "STONE_700" =>
    Color::STONE_700, "STONE7" => Color::STONE_700, "STONE_800" => Color::STONE_800,
    "STONE8" => Color::STONE_800, "STONE_900" => Color::STONE_900, "STONE9" =>
    Color::STONE_900, "STONE_950" => Color::STONE_950, "STONE9.5" => Color::STONE_950,
    "STONE95" => Color::STONE_950, "TEAL_50" => Color::TEAL_50, "TEAL0.5" =>
    Color::TEAL_50, "TEAL05" => Color::TEAL_50, "TEAL_100" => Color::TEAL_100, "TEAL1" =>
    Color::TEAL_100, "TEAL_200" => Color::TEAL_200, "TEAL2" => Color::TEAL_200,
    "TEAL_300" => Color::TEAL_300, "TEAL3" => Color::TEAL_300, "TEAL_400" =>
    Color::TEAL_400, "TEAL4" => Color::TEAL_400, "TEAL_500" => Color::TEAL_500, "TEAL5"
    => Color::TEAL_500, "TEAL_600" => Color::TEAL_600, "TEAL6" => Color::TEAL_600,
    "TEAL_700" => Color::TEAL_700, "TEAL7" => Color::TEAL_700, "TEAL_800" =>
    Color::TEAL_800, "TEAL8" => Color::TEAL_800, "TEAL_900" => Color::TEAL_900, "TEAL9"
    => Color::TEAL_900, "TEAL_950" => Color::TEAL_950, "TEAL9.5" => Color::TEAL_950,
    "TEAL95" => Color::TEAL_950, "VIOLET_50" => Color::VIOLET_50, "VIOLET0.5" =>
    Color::VIOLET_50, "VIOLET05" => Color::VIOLET_50, "VIOLET_100" => Color::VIOLET_100,
    "VIOLET1" => Color::VIOLET_100, "VIOLET_200" => Color::VIOLET_200, "VIOLET2" =>
    Color::VIOLET_200, "VIOLET_300" => Color::VIOLET_300, "VIOLET3" => Color::VIOLET_300,
    "VIOLET_400" => Color::VIOLET_400, "VIOLET4" => Color::VIOLET_400, "VIOLET_500" =>
    Color::VIOLET_500, "VIOLET5" => Color::VIOLET_500, "VIOLET_600" => Color::VIOLET_600,
    "VIOLET6" => Color::VIOLET_600, "VIOLET_700" => Color::VIOLET_700, "VIOLET7" =>
    Color::VIOLET_700, "VIOLET_800" => Color::VIOLET_800, "VIOLET8" => Color::VIOLET_800,
    "VIOLET_900" => Color::VIOLET_900, "VIOLET9" => Color::VIOLET_900, "VIOLET_950" =>
    Color::VIOLET_950, "VIOLET9.5" => Color::VIOLET_950, "VIOLET95" => Color::VIOLET_950,
    "YELLOW_50" => Color::YELLOW_50, "YELLOW0.5" => Color::YELLOW_50, "YELLOW05" =>
    Color::YELLOW_50, "YELLOW_100" => Color::YELLOW_100, "YELLOW1" => Color::YELLOW_100,
    "YELLOW_200" => Color::YELLOW_200, "YELLOW2" => Color::YELLOW_200, "YELLOW_300" =>
    Color::YELLOW_300, "YELLOW3" => Color::YELLOW_300, "YELLOW_400" => Color::YELLOW_400,
    "YELLOW4" => Color::YELLOW_400, "YELLOW_500" => Color::YELLOW_500, "YELLOW5" =>
    Color::YELLOW_500, "YELLOW_600" => Color::YELLOW_600, "YELLOW6" => Color::YELLOW_600,
    "YELLOW_700" => Color::YELLOW_700, "YELLOW7" => Color::YELLOW_700, "YELLOW_800" =>
    Color::YELLOW_800, "YELLOW8" => Color::YELLOW_800, "YELLOW_900" => Color::YELLOW_900,
    "YELLOW9" => Color::YELLOW_900, "YELLOW_950" => Color::YELLOW_950, "YELLOW9.5" =>
    Color::YELLOW_950, "YELLOW95" => Color::YELLOW_950, "ZINC_50" => Color::ZINC_50,
    "ZINC0.5" => Color::ZINC_50, "ZINC05" => Color::ZINC_50, "ZINC_100" =>
    Color::ZINC_100, "ZINC1" => Color::ZINC_100, "ZINC_200" => Color::ZINC_200, "ZINC2"
    => Color::ZINC_200, "ZINC_300" => Color::ZINC_300, "ZINC3" => Color::ZINC_300,
    "ZINC_400" => Color::ZINC_400, "ZINC4" => Color::ZINC_400, "ZINC_500" =>
    Color::ZINC_500, "ZINC5" => Color::ZINC_500, "ZINC_600" => Color::ZINC_600, "ZINC6"
    => Color::ZINC_600, "ZINC_700" => Color::ZINC_700, "ZINC7" => Color::ZINC_700,
    "ZINC_800" => Color::ZINC_800, "ZINC8" => Color::ZINC_800, "ZINC_900" =>
    Color::ZINC_900, "ZINC9" => Color::ZINC_900, "ZINC_950" => Color::ZINC_950, "ZINC9.5"
    => Color::ZINC_950, "ZINC95" => Color::ZINC_950,
};
