#[doc = "Register `dram_clk` reader"]
pub struct R(crate::R<DRAM_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRAM_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRAM_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRAM_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dram_clk` writer"]
pub struct W(crate::W<DRAM_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRAM_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DRAM_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRAM_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dram_div1` reader - Factor M"]
pub type DRAM_DIV1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dram_div1` writer - Factor M"]
pub type DRAM_DIV1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRAM_CLK_SPEC, u8, u8, 2, O>;
#[doc = "Field `dram_div2` reader - Factor N"]
pub type DRAM_DIV2_R = crate::FieldReader<u8, DRAM_DIV2_A>;
#[doc = "Factor N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRAM_DIV2_A {
    #[doc = "0: `0`"]
    N1 = 0,
    #[doc = "1: `1`"]
    N2 = 1,
    #[doc = "2: `10`"]
    N4 = 2,
    #[doc = "3: `11`"]
    N8 = 3,
}
impl From<DRAM_DIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: DRAM_DIV2_A) -> Self {
        variant as _
    }
}
impl DRAM_DIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRAM_DIV2_A {
        match self.bits {
            0 => DRAM_DIV2_A::N1,
            1 => DRAM_DIV2_A::N2,
            2 => DRAM_DIV2_A::N4,
            3 => DRAM_DIV2_A::N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `N1`"]
    #[inline(always)]
    pub fn is_n1(&self) -> bool {
        *self == DRAM_DIV2_A::N1
    }
    #[doc = "Checks if the value of the field is `N2`"]
    #[inline(always)]
    pub fn is_n2(&self) -> bool {
        *self == DRAM_DIV2_A::N2
    }
    #[doc = "Checks if the value of the field is `N4`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == DRAM_DIV2_A::N4
    }
    #[doc = "Checks if the value of the field is `N8`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == DRAM_DIV2_A::N8
    }
}
#[doc = "Field `dram_div2` writer - Factor N"]
pub type DRAM_DIV2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DRAM_CLK_SPEC, u8, DRAM_DIV2_A, 2, O>;
impl<'a, const O: u8> DRAM_DIV2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn n1(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n2(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut W {
        self.variant(DRAM_DIV2_A::N8)
    }
}
#[doc = "Field `clk_src_sel` reader - Clock Source Select"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_DDR = 0,
    #[doc = "1: `1`"]
    PLL_AUDIO1_DIV2 = 1,
    #[doc = "2: `10`"]
    PLL_PERI_2X = 2,
    #[doc = "3: `11`"]
    PLL_PERI_800M = 3,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_DDR),
            1 => Some(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2),
            2 => Some(CLK_SRC_SEL_A::PLL_PERI_2X),
            3 => Some(CLK_SRC_SEL_A::PLL_PERI_800M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_DDR`"]
    #[inline(always)]
    pub fn is_pll_ddr(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_DDR
    }
    #[doc = "Checks if the value of the field is `PLL_AUDIO1_DIV2`"]
    #[inline(always)]
    pub fn is_pll_audio1_div2(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_AUDIO1_DIV2
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_2X`"]
    #[inline(always)]
    pub fn is_pll_peri_2x(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_2X
    }
    #[doc = "Checks if the value of the field is `PLL_PERI_800M`"]
    #[inline(always)]
    pub fn is_pll_peri_800m(&self) -> bool {
        *self == CLK_SRC_SEL_A::PLL_PERI_800M
    }
}
#[doc = "Field `clk_src_sel` writer - Clock Source Select"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DRAM_CLK_SPEC, u8, CLK_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_ddr(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_DDR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_audio1_div2(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_AUDIO1_DIV2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pll_peri_2x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_2X)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pll_peri_800m(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_PERI_800M)
    }
}
#[doc = "Field `sdrclk_upd` reader - SDRCLK Configuration 0 Update"]
pub type SDRCLK_UPD_R = crate::BitReader<SDRCLK_UPD_A>;
#[doc = "SDRCLK Configuration 0 Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDRCLK_UPD_A {
    #[doc = "0: `0`"]
    INVALID = 0,
    #[doc = "1: `1`"]
    VALID = 1,
}
impl From<SDRCLK_UPD_A> for bool {
    #[inline(always)]
    fn from(variant: SDRCLK_UPD_A) -> Self {
        variant as u8 != 0
    }
}
impl SDRCLK_UPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDRCLK_UPD_A {
        match self.bits {
            false => SDRCLK_UPD_A::INVALID,
            true => SDRCLK_UPD_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == SDRCLK_UPD_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == SDRCLK_UPD_A::VALID
    }
}
#[doc = "Field `sdrclk_upd` writer - SDRCLK Configuration 0 Update"]
pub type SDRCLK_UPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRAM_CLK_SPEC, SDRCLK_UPD_A, O>;
impl<'a, const O: u8> SDRCLK_UPD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(SDRCLK_UPD_A::INVALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(SDRCLK_UPD_A::VALID)
    }
}
#[doc = "Field `clk_gating` reader - Gating Clock"]
pub type CLK_GATING_R = crate::BitReader<CLK_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_GATING_A {
        match self.bits {
            false => CLK_GATING_A::OFF,
            true => CLK_GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLK_GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CLK_GATING_A::ON
    }
}
#[doc = "Field `clk_gating` writer - Gating Clock"]
pub type CLK_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRAM_CLK_SPEC, CLK_GATING_A, O>;
impl<'a, const O: u8> CLK_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_GATING_A::ON)
    }
}
impl R {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn dram_div1(&self) -> DRAM_DIV1_R {
        DRAM_DIV1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn dram_div2(&self) -> DRAM_DIV2_R {
        DRAM_DIV2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    pub fn sdrclk_upd(&self) -> SDRCLK_UPD_R {
        SDRCLK_UPD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&self) -> CLK_GATING_R {
        CLK_GATING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Factor M"]
    #[inline(always)]
    pub fn dram_div1(&mut self) -> DRAM_DIV1_W<0> {
        DRAM_DIV1_W::new(self)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn dram_div2(&mut self) -> DRAM_DIV2_W<8> {
        DRAM_DIV2_W::new(self)
    }
    #[doc = "Bits 24:26 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<24> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 27 - SDRCLK Configuration 0 Update"]
    #[inline(always)]
    pub fn sdrclk_upd(&mut self) -> SDRCLK_UPD_W<27> {
        SDRCLK_UPD_W::new(self)
    }
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn clk_gating(&mut self) -> CLK_GATING_W<31> {
        CLK_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DRAM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dram_clk](index.html) module"]
pub struct DRAM_CLK_SPEC;
impl crate::RegisterSpec for DRAM_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dram_clk::R](R) reader structure"]
impl crate::Readable for DRAM_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dram_clk::W](W) writer structure"]
impl crate::Writable for DRAM_CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dram_clk to value 0"]
impl crate::Resettable for DRAM_CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
