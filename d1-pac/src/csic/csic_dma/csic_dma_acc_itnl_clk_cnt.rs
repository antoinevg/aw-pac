#[doc = "Register `csic_dma_acc_itnl_clk_cnt` reader"]
pub struct R(crate::R<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_acc_itnl_clk_cnt` writer"]
pub struct W(crate::W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>;
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
impl From<crate::W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `itnl_clk_cnt` reader - The instant value of internal frame clock counter.\n\nWhen frame done interrupt comes, the software can query this counter for judging whether it is the time for updating the double buffer address registers."]
pub type ITNL_CLK_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `acc_clk_cnt` reader - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
pub type ACC_CLK_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acc_clk_cnt` writer - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
pub type ACC_CLK_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - The instant value of internal frame clock counter.\n\nWhen frame done interrupt comes, the software can query this counter for judging whether it is the time for updating the double buffer address registers."]
    #[inline(always)]
    pub fn itnl_clk_cnt(&self) -> ITNL_CLK_CNT_R {
        ITNL_CLK_CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:31 - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn acc_clk_cnt(&self) -> ACC_CLK_CNT_R {
        ACC_CLK_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - The accumulated value of FRM_CLK_CNT for software frame rate statics. Every interrupt of frame is done, the software checks this accumulated value and clears it to 0. If the ACC_CLK_CNT is larger than 1, the software has lost frame.\n\nWhen frame done or vsync comes, ACC_CLK_CNT = ACC_CLK_CNT + 1, and cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn acc_clk_cnt(&mut self) -> ACC_CLK_CNT_W<24> {
        ACC_CLK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Accumulated And Internal Clock Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_acc_itnl_clk_cnt](index.html) module"]
pub struct CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_acc_itnl_clk_cnt::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_acc_itnl_clk_cnt::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_acc_itnl_clk_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_ACC_ITNL_CLK_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
