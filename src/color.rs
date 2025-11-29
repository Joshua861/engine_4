use bevy_math::Vec4;
use palette::{Hsl, IntoColor, LinSrgb, Oklch, Srgb};

pub mod u8;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    pub const fn new_u8(r: u8, g: u8, b: u8) -> Self {
        const fn convert(n: u8) -> f32 {
            (n * 255) as f32
        }
        Self::new(convert(r), convert(g), convert(b))
    }
    pub const fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        const fn convert(n: u8) -> f32 {
            n as f32 / 255.0
        }
        Self::from_rgba(convert(r), convert(g), convert(b), convert(a))
    }
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub const fn for_gpu(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub const fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
    pub fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }
    pub const fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    fn to_hsl(&self) -> (f32, f32, f32) {
        let lin_rgb = LinSrgb::new(self.r, self.g, self.b);
        let srgb: Srgb = lin_rgb.into_color();
        let hsl: Hsl = srgb.into_color();
        (
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        )
    }

    pub fn hex(hex: u32) -> Self {
        let red = (hex & 0xFF0000) >> 16;
        let green = (hex & 0x00FF00) >> 8;
        let blue = hex & 0x0000FF;

        Self::from_rgba_u8(red as u8, green as u8, blue as u8, 255)
    }

    pub fn hex_alpha(hex: u32) -> Self {
        let red = (hex & 0xFF000000) >> 24;
        let green = (hex & 0x00FF0000) >> 16;
        let blue = (hex & 0x0000FF00) >> 8;
        let alpha = hex & 0x000000FF;

        Self::from_rgba_u8(red as u8, green as u8, blue as u8, alpha as u8)
    }

    pub fn from_hsl_with_alpha(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        let hsl = Hsl::new(hue, saturation, lightness);
        let srgb: Srgb = hsl.into_color();
        let lin_rgb: LinSrgb = srgb.into_color();
        Self::from_rgba(lin_rgb.red, lin_rgb.green, lin_rgb.blue, alpha)
    }

    pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::from_hsl_with_alpha(hue, saturation, lightness, 1.0)
    }

    pub fn to_oklch(&self) -> (f32, f32, f32) {
        let lin_rgb = LinSrgb::new(self.r, self.g, self.b);
        let oklch: Oklch = lin_rgb.into_color();
        (oklch.l, oklch.chroma, oklch.hue.into_positive_degrees())
    }

    pub fn from_oklch_with_alpha(lightness: f32, chroma: f32, hue: f32, alpha: f32) -> Self {
        let oklch = Oklch::new(lightness, chroma, hue);
        let lin_rgb: LinSrgb = oklch.into_color();
        Self::from_rgba(lin_rgb.red, lin_rgb.green, lin_rgb.blue, alpha)
    }

    pub fn lighten(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l + factor * (1.0 - l)).clamp(0.0, 1.0);
        Self::from_hsl_with_alpha(h, s, new_l, self.a)
    }

    pub fn darken(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_l = (l - factor * l).clamp(0.0, 1.0);
        Self::from_hsl_with_alpha(h, s, new_l, self.a)
    }

    pub fn saturate(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s + factor * (1.0 - s)).clamp(0.0, 1.0);
        Self::from_hsl_with_alpha(h, new_s, l, self.a)
    }

    pub fn desaturate(self, factor: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_s = (s - factor * s).clamp(0.0, 1.0);
        Self::from_hsl_with_alpha(h, new_s, l, self.a)
    }

    pub fn hue_rotate(self, degrees: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        let new_h = (h + degrees).rem_euclid(360.0);
        Self::from_hsl_with_alpha(new_h, s, l, self.a)
    }

    pub fn lighten_oklch(self, factor: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_l = (l + factor * (1.0 - l)).clamp(0.0, 1.0);
        Self::from_oklch_with_alpha(new_l, c, h, self.a)
    }

    pub fn darken_oklch(self, factor: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_l = (l - factor * l).clamp(0.0, 1.0);
        Self::from_oklch_with_alpha(new_l, c, h, self.a)
    }

    pub fn hue_rotate_oklch(self, degrees: f32) -> Self {
        let (l, c, h) = self.to_oklch();
        let new_h = (h + degrees).rem_euclid(360.0);
        Self::from_oklch_with_alpha(l, c, new_h, self.a)
    }

    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::from_rgba(0.0, 0.0, 0.0, 0.0);
}

#[rustfmt::skip]
impl Color {
    pub const SLATE_50: Self = Self::new(0.9400013665754798, 0.9551989617830331, 0.9705415305631548);
    pub const SLATE_100: Self = Self::new(0.878307191251998, 0.9124754142945134, 0.9474050664454327);
    pub const SLATE_200: Self = Self::new(0.7599039165900536, 0.8077395851660172, 0.8741819568340998);
    pub const SLATE_300: Self = Self::new(0.5909385012191164, 0.6663452400721044, 0.7638211441879266);
    pub const SLATE_400: Self = Self::new(0.27904858538868743, 0.35680370765344765, 0.4847754927262069);
    pub const SLATE_500: Self = Self::new(0.12187093691275566, 0.17455309946514552, 0.27028325367330097);
    pub const SLATE_600: Self = Self::new(0.05969034116108278, 0.09126234474990905, 0.15045617336923184);
    pub const SLATE_700: Self = Self::new(0.031115587546566514, 0.05300981353171113, 0.09664706909652117);
    pub const SLATE_800: Self = Self::new(0.0120090985484631, 0.021991741087481007, 0.04645858006247726);
    pub const SLATE_900: Self = Self::new(0.0046135083757666995, 0.008546386120542758, 0.02420135296679099);
    pub const SLATE_950: Self = Self::new(0.0005750248252141058, 0.001801992989089289, 0.008898787567161852);
    pub const GRAY_50: Self = Self::new(0.9471382372706584, 0.9573034683102504, 0.9675307387112277);
    pub const GRAY_100: Self = Self::new(0.8960502515816096, 0.9048092598903361, 0.9224769508683622);
    pub const GRAY_200: Self = Self::new(0.7841376096813796, 0.8002125611030797, 0.8329355119089706);
    pub const GRAY_300: Self = Self::new(0.6365647892999851, 0.6663217670247163, 0.7124480508193525);
    pub const GRAY_400: Self = Self::new(0.318557060379478, 0.3564283433998119, 0.4272319139450453);
    pub const GRAY_500: Self = Self::new(0.143958084699357, 0.16829526016176644, 0.223885726873609);
    pub const GRAY_600: Self = Self::new(0.06783121652230038, 0.09096111108652145, 0.13016274288786106);
    pub const GRAY_700: Self = Self::new(0.036812376176001846, 0.05291290965928773, 0.0860727859473038);
    pub const GRAY_800: Self = Self::new(0.012861799719024358, 0.022166477508271484, 0.040383491695888014);
    pub const GRAY_900: Self = Self::new(0.005371561768053918, 0.0090788722752311, 0.021200188726686765);
    pub const GRAY_950: Self = Self::new(0.0008966506824431932, 0.002136790388712605, 0.006198735931159511);
    pub const ZINC_50: Self = Self::new(0.9556716250000004, 0.9556716249999997, 0.9556716249999999);
    pub const ZINC_100: Self = Self::new(0.9036362920616663, 0.903637101977269, 0.9099921462305894);
    pub const ZINC_200: Self = Self::new(0.776526835777584, 0.7765272040626082, 0.7996382862617033);
    pub const ZINC_300: Self = Self::new(0.6578668438862944, 0.657859114537089, 0.6890367724795103);
    pub const ZINC_400: Self = Self::new(0.345581709024783, 0.34551835199951325, 0.39757095224727496);
    pub const ZINC_500: Self = Self::new(0.16501920166277653, 0.16496672774375176, 0.19935567820381422);
    pub const ZINC_600: Self = Self::new(0.08416837445771143, 0.08411916297691685, 0.1078423629667849);
    pub const ZINC_700: Self = Self::new(0.049480202197592076, 0.04946530011597286, 0.06212182637100679);
    pub const ZINC_800: Self = Self::new(0.02027820043475683, 0.02027584357579072, 0.023423479299488577);
    pub const ZINC_900: Self = Self::new(0.009086902165316213, 0.009086392988180834, 0.01095180670231456);
    pub const ZINC_950: Self = Self::new(0.002737945885545722, 0.002736764680145174, 0.0034439960677028272);
    pub const NEUTRAL_50: Self = Self::new(0.9556716250000004, 0.9556716249999997, 0.9556716249999999);
    pub const NEUTRAL_100: Self = Self::new(0.9126730000000007, 0.9126729999999996, 0.912673);
    pub const NEUTRAL_200: Self = Self::new(0.7837774480000006, 0.7837774479999998, 0.7837774480000002);
    pub const NEUTRAL_300: Self = Self::new(0.6585030000000005, 0.6585029999999997, 0.6585030000000001);
    pub const NEUTRAL_400: Self = Self::new(0.3548949120000003, 0.35489491199999984, 0.3548949119999999);
    pub const NEUTRAL_500: Self = Self::new(0.17187961600000007, 0.17187961599999996, 0.17187961600000007);
    pub const NEUTRAL_600: Self = Self::new(0.0846045190000001, 0.08460451899999996, 0.08460451900000002);
    pub const NEUTRAL_700: Self = Self::new(0.05106481100000004, 0.051064810999999995, 0.051064811);
    pub const NEUTRAL_800: Self = Self::new(0.019465109, 0.01946510899999998, 0.019465108999999987);
    pub const NEUTRAL_900: Self = Self::new(0.008615125000000001, 0.008615124999999996, 0.008615124999999996);
    pub const NEUTRAL_950: Self = Self::new(0.0030486250000000006, 0.003048624999999998, 0.003048624999999999);
    pub const STONE_50: Self = Self::new(0.9562858806664226, 0.956286600685442, 0.9497107443817616);
    pub const STONE_100: Self = Self::new(0.9132686034176489, 0.9132694007789393, 0.9068924317971034);
    pub const STONE_200: Self = Self::new(0.8007565095108755, 0.7825368521584866, 0.7735164038509639);
    pub const STONE_300: Self = Self::new(0.6759680903025156, 0.65150883331355, 0.6354875907690248);
    pub const STONE_400: Self = Self::new(0.38281871200828865, 0.350016086620692, 0.3290792619848064);
    pub const STONE_500: Self = Self::new(0.1896492497124281, 0.1642305481900195, 0.14734676542316588);
    pub const STONE_600: Self = Self::new(0.09616071878551175, 0.08605657711416091, 0.07432452333910977);
    pub const STONE_700: Self = Self::new(0.05859406533877268, 0.05104579631531756, 0.044087225040646785);
    pub const STONE_800: Self = Self::new(0.022381528735562116, 0.018300427995881427, 0.017354377720805385);
    pub const STONE_900: Self = Self::new(0.011562952491759596, 0.00971336254424042, 0.00858384265803528);
    pub const STONE_950: Self = Self::new(0.0036679012073694986, 0.003046138677341473, 0.0027511210537268407);
    pub const RED_50: Self = Self::new(0.9928294533344858, 0.8891244503907021, 0.8891249600725629);
    pub const RED_100: Self = Self::new(0.9984102671170719, 0.7593004267427141, 0.7593662108271092);
    pub const RED_200: Self = Self::new(1.0, 0.5867552843730126, 0.5870634617158728);
    pub const RED_300: Self = Self::new(1.0, 0.360535553234547, 0.3626614966715039);
    pub const RED_400: Self = Self::new(1.0, 0.12675193856681835, 0.13558739193980454);
    pub const RED_500: Self = Self::new(0.9610095688472267, 0.024987571325519098, 0.03732420826094535);
    pub const RED_600: Self = Self::new(0.8002512914026426, 0.0, 0.0032715192310867267);
    pub const RED_700: Self = Self::new(0.5333110122519873, 0.0, 0.0022255687258870786);
    pub const RED_800: Self = Self::new(0.34508599421374947, 0.0022283732025938628, 0.005859837796871803);
    pub const RED_900: Self = Self::new(0.22262759413248565, 0.008873287458050131, 0.010129764842342213);
    pub const RED_950: Self = Self::new(0.061642417497145774, 0.0024452907741196682, 0.002670093992861722);
    pub const ORANGE_50: Self = Self::new(1.0, 0.9310392735123856, 0.8465659112449326);
    pub const ORANGE_100: Self = Self::new(1.0, 0.8457681629517466, 0.6605379516160578);
    pub const ORANGE_200: Self = Self::new(1.0, 0.6758764999997723, 0.3889574395259421);
    pub const ORANGE_300: Self = Self::new(1.0, 0.48036818666953074, 0.14283897134240098);
    pub const ORANGE_400: Self = Self::new(1.0, 0.24992335417453798, 0.001071274527622533);
    pub const ORANGE_500: Self = Self::new(0.9496103418993658, 0.16688610831715953, 0.0);
    pub const ORANGE_600: Self = Self::new(0.7840304354995702, 0.10449763169239193, 0.0);
    pub const ORANGE_700: Self = Self::new(0.5422408783973398, 0.04917381044067614, 0.0);
    pub const ORANGE_800: Self = Self::new(0.34789328945393944, 0.0262854953287822, 0.0);
    pub const ORANGE_900: Self = Self::new(0.20955069272464572, 0.023334109768409814, 0.003556070325073258);
    pub const ORANGE_950: Self = Self::new(0.05770528758068927, 0.006500172759943791, 0.0016954585184715313);
    pub const AMBER_50: Self = Self::new(1.0, 0.9651154732897442, 0.8275584308460651);
    pub const AMBER_100: Self = Self::new(0.993188855672863, 0.8965712257428442, 0.5664299330936392);
    pub const AMBER_200: Self = Self::new(0.9903135866967158, 0.7897985496314772, 0.23515680026174723);
    pub const AMBER_300: Self = Self::new(1.0, 0.6450440174519968, 0.029373467980211038);
    pub const AMBER_400: Self = Self::new(0.9757725870818409, 0.4932126379000067, 0.0);
    pub const AMBER_500: Self = Self::new(0.9238307664930733, 0.33747157104575815, 0.0);
    pub const AMBER_600: Self = Self::new(0.68770171807081, 0.1840003801885104, 0.0);
    pub const AMBER_700: Self = Self::new(0.4613368416552575, 0.08309840183601105, 0.0);
    pub const AMBER_800: Self = Self::new(0.3073496143216228, 0.044614003650333064, 0.0);
    pub const AMBER_900: Self = Self::new(0.1963868180350582, 0.03303168526535841, 0.0019220656967101624);
    pub const AMBER_950: Self = Self::new(0.06104877484640132, 0.009790493889248502, 0.0003612084305924944);
    pub const YELLOW_50: Self = Self::new(0.9899525600371317, 0.9724770644225634, 0.807503618275852);
    pub const YELLOW_100: Self = Self::new(0.9931498861217267, 0.9479969093190463, 0.5377936487460533);
    pub const YELLOW_200: Self = Self::new(0.9970093365099268, 0.8712711045332546, 0.23578194865964955);
    pub const YELLOW_300: Self = Self::new(1.0, 0.7412431310344926, 0.014486494494300811);
    pub const YELLOW_400: Self = Self::new(0.9622997930963095, 0.5770276489491186, 0.0);
    pub const YELLOW_500: Self = Self::new(0.8452146686319166, 0.4435385339630295, 0.0);
    pub const YELLOW_600: Self = Self::new(0.6015015542605783, 0.25063541798013955, 0.0);
    pub const YELLOW_700: Self = Self::new(0.3669942163863731, 0.11867944212065622, 0.0);
    pub const YELLOW_800: Self = Self::new(0.24879652220635579, 0.06968244264786141, 0.0);
    pub const YELLOW_900: Self = Self::new(0.171039472120256, 0.04788732945414958, 0.003166636388025468);
    pub const YELLOW_950: Self = Self::new(0.05575440207945748, 0.01415934394675905, 0.0013483412421366027);
    pub const LIME_50: Self = Self::new(0.9310153265922976, 0.9921022335509229, 0.7997987454541957);
    pub const LIME_100: Self = Self::new(0.8379495615543393, 0.9747119357836244, 0.5926558323719067);
    pub const LIME_200: Self = Self::new(0.6879750646426015, 0.9513541532580699, 0.31843105276866424);
    pub const LIME_300: Self = Self::new(0.496333354032384, 0.9010107267792795, 0.08236590489944308);
    pub const LIME_400: Self = Self::new(0.3228391706870927, 0.790205231407877, 0.0);
    pub const LIME_500: Self = Self::new(0.22689816173053173, 0.6121283394429926, 0.0);
    pub const LIME_600: Self = Self::new(0.12754630271452927, 0.371345543872719, 0.0);
    pub const LIME_700: Self = Self::new(0.06594098851807043, 0.20625977832833634, 0.0);
    pub const LIME_800: Self = Self::new(0.04591110873040873, 0.124661673415186, 0.0);
    pub const LIME_900: Self = Self::new(0.03533369026327776, 0.0874493843521654, 0.004423633870335841);
    pub const LIME_950: Self = Self::new(0.009881582116209707, 0.027571551449804468, 0.0008467395456928754);
    pub const GREEN_50: Self = Self::new(0.8718222746049414, 0.9823522769439171, 0.9050085672868351);
    pub const GREEN_100: Self = Self::new(0.7122070636882052, 0.9733910023380739, 0.7967390239008517);
    pub const GREEN_200: Self = Self::new(0.4845368381274443, 0.9358635911249136, 0.624084329357284);
    pub const GREEN_300: Self = Self::new(0.1972759515612579, 0.8822651203925421, 0.3891789119527681);
    pub const GREEN_400: Self = Self::new(0.0015379710098909205, 0.7394402735435098, 0.16943880236374098);
    pub const GREEN_500: Self = Self::new(0.0, 0.5822851702253343, 0.08121351100205147);
    pub const GREEN_600: Self = Self::new(0.0, 0.3817433902901538, 0.047729124722795335);
    pub const GREEN_700: Self = Self::new(0.0, 0.2234117738712372, 0.03624391192491793);
    pub const GREEN_800: Self = Self::new(0.00044860718121511947, 0.13390555810322438, 0.02965991992938835);
    pub const GREEN_900: Self = Self::new(0.0040379478177908, 0.08825594583283301, 0.024560137471514823);
    pub const GREEN_950: Self = Self::new(0.0009284620309370642, 0.027490944319150862, 0.007650682872915743);
    pub const EMERALD_50: Self = Self::new(0.8364010610157682, 0.9819684883130622, 0.9117955194934144);
    pub const EMERALD_100: Self = Self::new(0.6311661468137477, 0.957029330346872, 0.7805368117028879);
    pub const EMERALD_200: Self = Self::new(0.3729683092847678, 0.9024086106287106, 0.626657984943594);
    pub const EMERALD_300: Self = Self::new(0.1127108674169125, 0.8176050319157391, 0.46057527251106534);
    pub const EMERALD_400: Self = Self::new(0.0, 0.6605510379103182, 0.2872125940171974);
    pub const EMERALD_500: Self = Self::new(0.0, 0.5052269002003122, 0.2042534615968416);
    pub const EMERALD_600: Self = Self::new(0.0, 0.31730963711539906, 0.1321571241077553);
    pub const EMERALD_700: Self = Self::new(0.0, 0.19443240541590465, 0.09096153395747436);
    pub const EMERALD_800: Self = Self::new(0.0, 0.11779036469275758, 0.05950612331453248);
    pub const EMERALD_900: Self = Self::new(0.0, 0.07731578892778188, 0.043170726034647174);
    pub const EMERALD_950: Self = Self::new(0.0, 0.025479818013103013, 0.01588453712426307);
    pub const TEAL_50: Self = Self::new(0.873652797484321, 0.9830651183423892, 0.9571362250266289);
    pub const TEAL_100: Self = Self::new(0.598382661043832, 0.9681039126755709, 0.8809495916540486);
    pub const TEAL_200: Self = Self::new(0.3036176420056011, 0.9275544250986272, 0.7767404429167024);
    pub const TEAL_300: Self = Self::new(0.061742704242517876, 0.8427938829908821, 0.6621821208217904);
    pub const TEAL_400: Self = Self::new(0.0, 0.6653088725319771, 0.5121719247926053);
    pub const TEAL_500: Self = Self::new(0.0, 0.49697144130835275, 0.3863887162658931);
    pub const TEAL_600: Self = Self::new(0.0, 0.3061640736638551, 0.25019230426413497);
    pub const TEAL_700: Self = Self::new(0.0, 0.18646045946254072, 0.15871000038661442);
    pub const TEAL_800: Self = Self::new(0.0, 0.11481748044870423, 0.10175727795727756);
    pub const TEAL_900: Self = Self::new(0.0032544676533650327, 0.0773500139269712, 0.06922319373919665);
    pub const TEAL_950: Self = Self::new(0.000628595430754544, 0.028552393993249114, 0.027441008906914508);
    pub const CYAN_50: Self = Self::new(0.8378006580705045, 0.9909755483821523, 0.999929793627783);
    pub const CYAN_100: Self = Self::new(0.619662862539516, 0.9564131580353388, 0.9921380378463978);
    pub const CYAN_200: Self = Self::new(0.36300937301601843, 0.9015776865374866, 0.9822469778733538);
    pub const CYAN_300: Self = Self::new(0.0871438220587919, 0.8214620014212903, 0.9804534241681679);
    pub const CYAN_400: Self = Self::new(0.0, 0.6512478476132039, 0.8920568948159185);
    pub const CYAN_500: Self = Self::new(0.0, 0.47977035464552137, 0.7056152358037138);
    pub const CYAN_600: Self = Self::new(0.0, 0.28925611921823, 0.48080839322509833);
    pub const CYAN_700: Self = Self::new(0.0, 0.17806587666003199, 0.2993474352126938);
    pub const CYAN_800: Self = Self::new(0.0, 0.11355435147422695, 0.18838699832481207);
    pub const CYAN_900: Self = Self::new(0.005285962003854866, 0.07660467087097989, 0.12852434677807026);
    pub const CYAN_950: Self = Self::new(0.0016399317246865337, 0.03330674369301001, 0.05918007257746619);
    pub const SKY_50: Self = Self::new(0.8685571636181125, 0.947575601050848, 1.0);
    pub const SKY_100: Self = Self::new(0.7399619900213945, 0.8877498750838709, 0.9950270615053961);
    pub const SKY_200: Self = Self::new(0.47974056407065124, 0.7927607244395868, 0.9932165863728917);
    pub const SKY_300: Self = Self::new(0.17326660348193368, 0.6588959069414809, 1.0);
    pub const SKY_400: Self = Self::new(0.0, 0.5016031428733327, 1.0);
    pub const SKY_500: Self = Self::new(0.0, 0.3797507831310332, 0.9052370734590842);
    pub const SKY_600: Self = Self::new(0.0, 0.2312426365617907, 0.637932503083689);
    pub const SKY_700: Self = Self::new(0.0, 0.1411690062089492, 0.3935136655119903);
    pub const SKY_800: Self = Self::new(0.0, 0.09989718494368895, 0.2528051373548286);
    pub const SKY_900: Self = Self::new(0.0005227045314489115, 0.06851388834438892, 0.16352028970250315);
    pub const SKY_950: Self = Self::new(0.001533667219472401, 0.02831890432360846, 0.0686521476580365);
    pub const BLUE_50: Self = Self::new(0.8624994608730726, 0.9201265311271156, 0.9974355119429067);
    pub const BLUE_100: Self = Self::new(0.7072916536073198, 0.8231521819409794, 0.993801923778675);
    pub const BLUE_200: Self = Self::new(0.5149098728002721, 0.7080785895260114, 1.0);
    pub const BLUE_300: Self = Self::new(0.2702110409466122, 0.5593372778063967, 1.0);
    pub const BLUE_400: Self = Self::new(0.0813069371302313, 0.3616791359702097, 1.0);
    pub const BLUE_500: Self = Self::new(0.0243408303830297, 0.21224020120545617, 1.0);
    pub const BLUE_600: Self = Self::new(0.0076660075607791756, 0.10928106049575825, 0.9692130672773918);
    pub const BLUE_700: Self = Self::new(0.006929701789625542, 0.06332969653120488, 0.7909281375741156);
    pub const BLUE_800: Self = Self::new(0.009988904235744916, 0.04461895355403514, 0.48132802786535556);
    pub const BLUE_900: Self = Self::new(0.011486153265743176, 0.040445472765134945, 0.2711888871519978);
    pub const BLUE_950: Self = Self::new(0.008158526068428575, 0.018067030415149832, 0.09217158463994851);
    pub const INDIGO_50: Self = Self::new(0.8551196914840783, 0.8881756769370186, 1.0);
    pub const INDIGO_100: Self = Self::new(0.7445705913536866, 0.7991344606337119, 1.0);
    pub const INDIGO_200: Self = Self::new(0.567782607964175, 0.6438860365877992, 1.0);
    pub const INDIGO_300: Self = Self::new(0.36614161418202895, 0.45102837635466586, 1.0);
    pub const INDIGO_400: Self = Self::new(0.20296439824021695, 0.239411177032814, 1.0);
    pub const INDIGO_500: Self = Self::new(0.12071725949527312, 0.11401140961669608, 1.0);
    pub const INDIGO_600: Self = Self::new(0.07874491147056778, 0.04122439496063337, 0.9249590774370772);
    pub const INDIGO_700: Self = Self::new(0.05681256607471136, 0.026274396720766764, 0.6826278027366689);
    pub const INDIGO_800: Self = Self::new(0.03811657491496867, 0.02278019921951718, 0.41145441459100623);
    pub const INDIGO_900: Self = Self::new(0.03056200552091934, 0.02521240679526046, 0.2356267199281135);
    pub const INDIGO_950: Self = Self::new(0.012890599582922678, 0.010435866637401973, 0.07354588289194511);
    pub const VIOLET_50: Self = Self::new(0.9126848743189352, 0.8959239077183477, 0.9992346706664805);
    pub const VIOLET_100: Self = Self::new(0.8461102959816982, 0.8133991892421287, 0.9933438798484293);
    pub const VIOLET_200: Self = Self::new(0.722819534924465, 0.6700233038053552, 1.0);
    pub const VIOLET_300: Self = Self::new(0.5543299822238934, 0.4536139741697419, 1.0);
    pub const VIOLET_400: Self = Self::new(0.38343614328876274, 0.23005177117105255, 1.0);
    pub const VIOLET_500: Self = Self::new(0.269505253249386, 0.08255679918915013, 1.0);
    pub const VIOLET_600: Self = Self::new(0.21395890339793672, 0.016072111923988018, 0.9909206377182606);
    pub const VIOLET_700: Self = Self::new(0.16239958033477142, 0.002432135779871669, 0.8001774318484904);
    pub const VIOLET_800: Self = Self::new(0.10988635097012328, 0.004452791604566287, 0.5276869829674711);
    pub const VIOLET_900: Self = Self::new(0.07446957686478134, 0.00841595188320403, 0.3229009710385694);
    pub const VIOLET_950: Self = Self::new(0.027924969721626808, 0.004017592408333003, 0.13736410838173377);
    pub const PURPLE_50: Self = Self::new(0.9560786681653385, 0.9137993893663617, 0.9994872661525143);
    pub const PURPLE_100: Self = Self::new(0.8954562731728074, 0.8055036759683187, 0.9999484551382523);
    pub const PURPLE_200: Self = Self::new(0.8174496084713141, 0.661435944324815, 1.0);
    pub const PURPLE_300: Self = Self::new(0.7001835986287928, 0.4448535920850055, 1.0);
    pub const PURPLE_400: Self = Self::new(0.5375505268917168, 0.1958184791702446, 1.0);
    pub const PURPLE_500: Self = Self::new(0.4171719661827128, 0.06188687745194293, 1.0);
    pub const PURPLE_600: Self = Self::new(0.3148702184711962, 0.005074902622424757, 0.9581144535446198);
    pub const PURPLE_700: Self = Self::new(0.2233245029064282, 0.0, 0.7054660543162319);
    pub const PURPLE_800: Self = Self::new(0.15486738853641582, 0.005602354732079023, 0.43492186635602065);
    pub const PURPLE_900: Self = Self::new(0.10091196766321084, 0.008163748201648663, 0.2582806959352153);
    pub const PURPLE_950: Self = Self::new(0.044938946140423375, 0.0009117751819138712, 0.13382996544334677);
    pub const FUCHSIA_50: Self = Self::new(0.9805705403563024, 0.9044616347979103, 0.9979705761619162);
    pub const FUCHSIA_100: Self = Self::new(0.9585437227939467, 0.8052016416016782, 1.0);
    pub const FUCHSIA_200: Self = Self::new(0.9199753213435149, 0.6266906410822893, 1.0);
    pub const FUCHSIA_300: Self = Self::new(0.9008824819662637, 0.38954489843262063, 1.0);
    pub const FUCHSIA_400: Self = Self::new(0.8479610776108222, 0.1455140555215783, 1.0);
    pub const FUCHSIA_500: Self = Self::new(0.7555186052678735, 0.023417496993577584, 0.9651043228093908);
    pub const FUCHSIA_600: Self = Self::new(0.5764127681426516, 0.0, 0.7322667401505699);
    pub const FUCHSIA_700: Self = Self::new(0.3911076229074274, 0.0, 0.47397928985928356);
    pub const FUCHSIA_800: Self = Self::new(0.2543780655426999, 0.00040020786094602766, 0.2976951845708536);
    pub const FUCHSIA_900: Self = Self::new(0.16895906989762022, 0.006613169321090327, 0.18697148176270673);
    pub const FUCHSIA_950: Self = Self::new(0.07024484883349441, 0.00011349986191483852, 0.07860710428797156);
    pub const PINK_50: Self = Self::new(0.9803619421769263, 0.8870712534208944, 0.9372761120414108);
    pub const PINK_100: Self = Self::new(0.9744164596626683, 0.7978930097109168, 0.8962765978034295);
    pub const PINK_200: Self = Self::new(0.9721897461746787, 0.6192793187473759, 0.8088089776896648);
    pub const PINK_300: Self = Self::new(0.9865932683116975, 0.37551786894228184, 0.668706859784262);
    pub const PINK_400: Self = Self::new(0.9675744341699404, 0.12735840256419897, 0.4679831474470388);
    pub const PINK_500: Self = Self::new(0.9239686704999738, 0.03252783200421816, 0.3236174804520982);
    pub const PINK_600: Self = Self::new(0.790058879178628, 0.0, 0.1817231604577101);
    pub const PINK_700: Self = Self::new(0.5668103535985467, 0.0, 0.10587680702433888);
    pub const PINK_800: Self = Self::new(0.3654774948532759, 0.0, 0.07227410690415055);
    pub const PINK_900: Self = Self::new(0.23809376413074118, 0.005238441900679702, 0.05553208926243074);
    pub const PINK_950: Self = Self::new(0.0824707884186615, 0.0011674697435296794, 0.017350439440261024);
    pub const ROSE_50: Self = Self::new(0.9977791744758353, 0.8788422536660386, 0.8870404156402807);
    pub const ROSE_100: Self = Self::new(1.0, 0.7744546054353062, 0.7900439896143062);
    pub const ROSE_200: Self = Self::new(1.0, 0.6057063309016345, 0.6482296470302574);
    pub const ROSE_300: Self = Self::new(1.0, 0.3546035988977616, 0.41936978791645685);
    pub const ROSE_400: Self = Self::new(1.0, 0.1251946619530587, 0.20883198207023262);
    pub const ROSE_500: Self = Self::new(1.0, 0.014276366333957565, 0.09405755045051417);
    pub const ROSE_600: Self = Self::new(0.8311944990391782, 0.0, 0.05140412798594048);
    pub const ROSE_700: Self = Self::new(0.5649455460099237, 0.0, 0.03681130156774573);
    pub const ROSE_800: Self = Self::new(0.37578920080931333, 0.0, 0.03685734103560458);
    pub const ROSE_900: Self = Self::new(0.25756876776980975, 0.0023249759686472895, 0.03653672606147411);
    pub const ROSE_950: Self = Self::new(0.0748068134880863, 0.0006826491618721138, 0.009392106875841258);
}
