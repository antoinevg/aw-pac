#[doc = "Register `csic_dma_buf_th` reader"]
pub struct R(crate::R<CSIC_DMA_BUF_TH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_BUF_TH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_BUF_TH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_BUF_TH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_buf_th` writer"]
pub struct W(crate::W<CSIC_DMA_BUF_TH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_BUF_TH_SPEC>;
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
impl From<crate::W<CSIC_DMA_BUF_TH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_BUF_TH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csic_dma_buf_addr_fifo_threshold` reader - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `csic_dma_buf_addr_fifo_threshold` writer - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_BUF_TH_SPEC, u8, u8, 6, O>;
#[doc = "Field `csic_dma_stored_frm_threshold` reader - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_STORED_FRM_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `csic_dma_stored_frm_threshold` writer - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_STORED_FRM_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_BUF_TH_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_threshold(&self) -> CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R {
        CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_stored_frm_threshold(&self) -> CSIC_DMA_STORED_FRM_THRESHOLD_R {
        CSIC_DMA_STORED_FRM_THRESHOLD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_threshold(&mut self) -> CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W<0> {
        CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 16:21 - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_stored_frm_threshold(&mut self) -> CSIC_DMA_STORED_FRM_THRESHOLD_W<16> {
        CSIC_DMA_STORED_FRM_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA BUF Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_buf_th](index.html) module"]
pub struct CSIC_DMA_BUF_TH_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_TH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_buf_th::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_TH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_buf_th::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_TH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_buf_th to value 0x0020_0000"]
impl crate::Resettable for CSIC_DMA_BUF_TH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}
