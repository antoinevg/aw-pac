#[doc = "Register `tve_low_pass_gain` reader"]
pub struct R(crate::R<TVE_LOW_PASS_GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LOW_PASS_GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LOW_PASS_GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LOW_PASS_GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_low_pass_gain` writer"]
pub struct W(crate::W<TVE_LOW_PASS_GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LOW_PASS_GAIN_SPEC>;
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
impl From<crate::W<TVE_LOW_PASS_GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LOW_PASS_GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain` reader - Peaking gain setting."]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain` writer - Peaking gain setting."]
pub type GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LOW_PASS_GAIN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Peaking gain setting."]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Peaking gain setting."]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W<0> {
        GAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Low Pass Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_low_pass_gain](index.html) module"]
pub struct TVE_LOW_PASS_GAIN_SPEC;
impl crate::RegisterSpec for TVE_LOW_PASS_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_low_pass_gain::R](R) reader structure"]
impl crate::Readable for TVE_LOW_PASS_GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_low_pass_gain::W](W) writer structure"]
impl crate::Writable for TVE_LOW_PASS_GAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tve_low_pass_gain to value 0"]
impl crate::Resettable for TVE_LOW_PASS_GAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
