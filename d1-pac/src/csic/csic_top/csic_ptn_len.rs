#[doc = "Register `csic_ptn_len` reader"]
pub struct R(crate::R<CSIC_PTN_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_PTN_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_PTN_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_PTN_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_ptn_len` writer"]
pub struct W(crate::W<CSIC_PTN_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_PTN_LEN_SPEC>;
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
impl From<crate::W<CSIC_PTN_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_PTN_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ptn_len` reader - The pattern length in byte when generating pattern."]
pub type PTN_LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ptn_len` writer - The pattern length in byte when generating pattern."]
pub type PTN_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_PTN_LEN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The pattern length in byte when generating pattern."]
    #[inline(always)]
    pub fn ptn_len(&self) -> PTN_LEN_R {
        PTN_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The pattern length in byte when generating pattern."]
    #[inline(always)]
    pub fn ptn_len(&mut self) -> PTN_LEN_W<0> {
        PTN_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC Pattern Generation Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_ptn_len](index.html) module"]
pub struct CSIC_PTN_LEN_SPEC;
impl crate::RegisterSpec for CSIC_PTN_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_ptn_len::R](R) reader structure"]
impl crate::Readable for CSIC_PTN_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_ptn_len::W](W) writer structure"]
impl crate::Writable for CSIC_PTN_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_ptn_len to value 0"]
impl crate::Resettable for CSIC_PTN_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
