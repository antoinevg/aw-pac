#[doc = "Register `ac_dac_drc_lhpfc` reader"]
pub struct R(crate::R<AC_DAC_DRC_LHPFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_DAC_DRC_LHPFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_DAC_DRC_LHPFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_DAC_DRC_LHPFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_dac_drc_lhpfc` writer"]
pub struct W(crate::W<AC_DAC_DRC_LHPFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_DAC_DRC_LHPFC_SPEC>;
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
impl From<crate::W<AC_DAC_DRC_LHPFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_DAC_DRC_LHPFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lhpfc` reader - HPF coefficient setting and the data is 3.24 format."]
pub type LHPFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `lhpfc` writer - HPF coefficient setting and the data is 3.24 format."]
pub type LHPFC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_DAC_DRC_LHPFC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    pub fn lhpfc(&self) -> LHPFC_R {
        LHPFC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HPF coefficient setting and the data is 3.24 format."]
    #[inline(always)]
    pub fn lhpfc(&mut self) -> LHPFC_W<0> {
        LHPFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC DRC Low HPF Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_dac_drc_lhpfc](index.html) module"]
pub struct AC_DAC_DRC_LHPFC_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LHPFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_dac_drc_lhpfc::R](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LHPFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_dac_drc_lhpfc::W](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LHPFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ac_dac_drc_lhpfc to value 0xfac1"]
impl crate::Resettable for AC_DAC_DRC_LHPFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfac1
    }
}
