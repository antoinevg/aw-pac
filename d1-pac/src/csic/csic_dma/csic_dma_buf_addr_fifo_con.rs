#[doc = "Register `csic_dma_buf_addr_fifo_con` reader"]
pub struct R(crate::R<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_buf_addr_fifo_con` writer"]
pub struct W(crate::W<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
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
impl From<crate::W<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csic_dma_buf_addr_fifo_content[0-2]` reader - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub unsafe fn csic_dma_buf_addr_fifo_content(&self, n: u8) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> (n * 8)) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo0_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo1_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo2_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA BUF Address FIFO Content Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_buf_addr_fifo_con](index.html) module"]
pub struct CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_buf_addr_fifo_con::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_buf_addr_fifo_con::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_buf_addr_fifo_con to value 0"]
impl crate::Resettable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
