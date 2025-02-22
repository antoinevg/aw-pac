#[doc = "Register `csic_dma_frm_clk_cnt` reader"]
pub struct R(crate::R<CSIC_DMA_FRM_CLK_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_FRM_CLK_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_FRM_CLK_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_FRM_CLK_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_frm_clk_cnt` writer"]
pub struct W(crate::W<CSIC_DMA_FRM_CLK_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_FRM_CLK_CNT_SPEC>;
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
impl From<crate::W<CSIC_DMA_FRM_CLK_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_FRM_CLK_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frm_clk_cnt` reader - Counter value between every frame. For instant hardware frame rate statics.\n\nThe internal counter is added by one every 12 MHz clock cycle. When frame done or vsync comes, the internal counter value is sampled to FRM_CLK_CNT, and cleared to 0."]
pub type FRM_CLK_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Counter value between every frame. For instant hardware frame rate statics.\n\nThe internal counter is added by one every 12 MHz clock cycle. When frame done or vsync comes, the internal counter value is sampled to FRM_CLK_CNT, and cleared to 0."]
    #[inline(always)]
    pub fn frm_clk_cnt(&self) -> FRM_CLK_CNT_R {
        FRM_CLK_CNT_R::new((self.bits & 0x00ff_ffff) as u32)
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
#[doc = "CSIC DMA Frame Clock Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_frm_clk_cnt](index.html) module"]
pub struct CSIC_DMA_FRM_CLK_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FRM_CLK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_frm_clk_cnt::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_FRM_CLK_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_frm_clk_cnt::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_FRM_CLK_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_frm_clk_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_FRM_CLK_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
