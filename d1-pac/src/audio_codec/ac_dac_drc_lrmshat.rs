#[doc = "Register `ac_dac_drc_lrmshat` reader"]
pub struct R(crate::R<AC_DAC_DRC_LRMSHAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DRC_LRMSHAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DRC_LRMSHAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DRC_LRMSHAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_drc_lrmshat` writer"]
pub struct W(crate::W<AC_DAC_DRC_LRMSHAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DRC_LRMSHAT_SPEC>;
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
impl From<crate::W<AC_DAC_DRC_LRMSHAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DRC_LRMSHAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dac_drc_lrmshat` reader - The left RMS filter average time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tav). The format is 3.24. (The default value is 10 ms)"]
pub type DAC_DRC_LRMSHAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dac_drc_lrmshat` writer - The left RMS filter average time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tav). The format is 3.24. (The default value is 10 ms)"]
pub type DAC_DRC_LRMSHAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DRC_LRMSHAT_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - The left RMS filter average time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tav). The format is 3.24. (The default value is 10 ms)"]
    #[inline(always)]
    pub fn dac_drc_lrmshat(&self) -> DAC_DRC_LRMSHAT_R {
        DAC_DRC_LRMSHAT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The left RMS filter average time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tav). The format is 3.24. (The default value is 10 ms)"]
    #[inline(always)]
    pub fn dac_drc_lrmshat(&mut self) -> DAC_DRC_LRMSHAT_W<0> {
        DAC_DRC_LRMSHAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DRC Left RMS Filter High Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_drc_lrmshat](index.html) module"]
pub struct AC_DAC_DRC_LRMSHAT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LRMSHAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_drc_lrmshat::R](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LRMSHAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_drc_lrmshat::W](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LRMSHAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_dac_drc_lrmshat to value 0x01"]
impl crate::Resettable for AC_DAC_DRC_LRMSHAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
