#[doc = "Register `ledc_data_finish_cnt` reader"]
pub struct R(crate::R<LEDC_DATA_FINISH_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_DATA_FINISH_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_DATA_FINISH_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_DATA_FINISH_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ledc_data_finish_cnt` writer"]
pub struct W(crate::W<LEDC_DATA_FINISH_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDC_DATA_FINISH_CNT_SPEC>;
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
impl From<crate::W<LEDC_DATA_FINISH_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDC_DATA_FINISH_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `led_data_finish_cnt` reader - "]
pub type LED_DATA_FINISH_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `led_wait_data_time` reader - "]
pub type LED_WAIT_DATA_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `led_wait_data_time` writer - "]
pub type LED_WAIT_DATA_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LEDC_DATA_FINISH_CNT_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn led_data_finish_cnt(&self) -> LED_DATA_FINISH_CNT_R {
        LED_DATA_FINISH_CNT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn led_wait_data_time(&self) -> LED_WAIT_DATA_TIME_R {
        LED_WAIT_DATA_TIME_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn led_wait_data_time(&mut self) -> LED_WAIT_DATA_TIME_W<16> {
        LED_WAIT_DATA_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC Data Finish Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_data_finish_cnt](index.html) module"]
pub struct LEDC_DATA_FINISH_CNT_SPEC;
impl crate::RegisterSpec for LEDC_DATA_FINISH_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_data_finish_cnt::R](R) reader structure"]
impl crate::Readable for LEDC_DATA_FINISH_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledc_data_finish_cnt::W](W) writer structure"]
impl crate::Writable for LEDC_DATA_FINISH_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ledc_data_finish_cnt to value 0"]
impl crate::Resettable for LEDC_DATA_FINISH_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
