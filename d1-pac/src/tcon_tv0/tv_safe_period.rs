#[doc = "Register `tv_safe_period` reader"]
pub struct R(crate::R<TV_SAFE_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_SAFE_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_SAFE_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_SAFE_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_safe_period` writer"]
pub struct W(crate::W<TV_SAFE_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_SAFE_PERIOD_SPEC>;
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
impl From<crate::W<TV_SAFE_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_SAFE_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `safe_period_mode` reader - Safe Period Mode"]
pub type SAFE_PERIOD_MODE_R = crate::FieldReader<u8, SAFE_PERIOD_MODE_A>;
#[doc = "Safe Period Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAFE_PERIOD_MODE_A {
    #[doc = "0: unsafe"]
    UNSAFE = 0,
    #[doc = "1: safe"]
    SAFE = 1,
    #[doc = "2: safe at LINE_BUF_CURR_NUM > SAFE_PERIOD_FIFO_NUM"]
    SAFE_LINE_BUF_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM = 2,
    #[doc = "3: safe at 2 and safe at sync active"]
    SAFE_2_SYNC_ACTIVE = 3,
    #[doc = "4: safe at line"]
    SAFE_LINE = 4,
}
impl From<SAFE_PERIOD_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAFE_PERIOD_MODE_A) -> Self {
        variant as _
    }
}
impl SAFE_PERIOD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAFE_PERIOD_MODE_A> {
        match self.bits {
            0 => Some(SAFE_PERIOD_MODE_A::UNSAFE),
            1 => Some(SAFE_PERIOD_MODE_A::SAFE),
            2 => Some(SAFE_PERIOD_MODE_A::SAFE_LINE_BUF_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM),
            3 => Some(SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE),
            4 => Some(SAFE_PERIOD_MODE_A::SAFE_LINE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNSAFE`"]
    #[inline(always)]
    pub fn is_unsafe(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::UNSAFE
    }
    #[doc = "Checks if the value of the field is `SAFE`"]
    #[inline(always)]
    pub fn is_safe(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE
    }
    #[doc = "Checks if the value of the field is `SAFE_LINE_BUF_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM`"]
    #[inline(always)]
    pub fn is_safe_line_buf_curr_num_gt_safe_period_fifo_num(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_LINE_BUF_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM
    }
    #[doc = "Checks if the value of the field is `SAFE_2_SYNC_ACTIVE`"]
    #[inline(always)]
    pub fn is_safe_2_sync_active(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE
    }
    #[doc = "Checks if the value of the field is `SAFE_LINE`"]
    #[inline(always)]
    pub fn is_safe_line(&self) -> bool {
        *self == SAFE_PERIOD_MODE_A::SAFE_LINE
    }
}
#[doc = "Field `safe_period_mode` writer - Safe Period Mode"]
pub type SAFE_PERIOD_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_SAFE_PERIOD_SPEC, u8, SAFE_PERIOD_MODE_A, 3, O>;
impl<'a, const O: u8> SAFE_PERIOD_MODE_W<'a, O> {
    #[doc = "unsafe"]
    #[inline(always)]
    pub fn unsafe_(self) -> &'a mut W {
        self.variant(SAFE_PERIOD_MODE_A::UNSAFE)
    }
    #[doc = "safe"]
    #[inline(always)]
    pub fn safe(self) -> &'a mut W {
        self.variant(SAFE_PERIOD_MODE_A::SAFE)
    }
    #[doc = "safe at LINE_BUF_CURR_NUM > SAFE_PERIOD_FIFO_NUM"]
    #[inline(always)]
    pub fn safe_line_buf_curr_num_gt_safe_period_fifo_num(self) -> &'a mut W {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_LINE_BUF_CURR_NUM_GT_SAFE_PERIOD_FIFO_NUM)
    }
    #[doc = "safe at 2 and safe at sync active"]
    #[inline(always)]
    pub fn safe_2_sync_active(self) -> &'a mut W {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_2_SYNC_ACTIVE)
    }
    #[doc = "safe at line"]
    #[inline(always)]
    pub fn safe_line(self) -> &'a mut W {
        self.variant(SAFE_PERIOD_MODE_A::SAFE_LINE)
    }
}
#[doc = "Field `safe_period_line` reader - Safe Period Line"]
pub type SAFE_PERIOD_LINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `safe_period_line` writer - Safe Period Line"]
pub type SAFE_PERIOD_LINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_SAFE_PERIOD_SPEC, u16, u16, 12, O>;
#[doc = "Field `safe_period_fifo_num` reader - Safe Period FIFO Number"]
pub type SAFE_PERIOD_FIFO_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `safe_period_fifo_num` writer - Safe Period FIFO Number"]
pub type SAFE_PERIOD_FIFO_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_SAFE_PERIOD_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:2 - Safe Period Mode"]
    #[inline(always)]
    pub fn safe_period_mode(&self) -> SAFE_PERIOD_MODE_R {
        SAFE_PERIOD_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:15 - Safe Period Line"]
    #[inline(always)]
    pub fn safe_period_line(&self) -> SAFE_PERIOD_LINE_R {
        SAFE_PERIOD_LINE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - Safe Period FIFO Number"]
    #[inline(always)]
    pub fn safe_period_fifo_num(&self) -> SAFE_PERIOD_FIFO_NUM_R {
        SAFE_PERIOD_FIFO_NUM_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Safe Period Mode"]
    #[inline(always)]
    pub fn safe_period_mode(&mut self) -> SAFE_PERIOD_MODE_W<0> {
        SAFE_PERIOD_MODE_W::new(self)
    }
    #[doc = "Bits 4:15 - Safe Period Line"]
    #[inline(always)]
    pub fn safe_period_line(&mut self) -> SAFE_PERIOD_LINE_W<4> {
        SAFE_PERIOD_LINE_W::new(self)
    }
    #[doc = "Bits 16:28 - Safe Period FIFO Number"]
    #[inline(always)]
    pub fn safe_period_fifo_num(&mut self) -> SAFE_PERIOD_FIFO_NUM_W<16> {
        SAFE_PERIOD_FIFO_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Safe Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_safe_period](index.html) module"]
pub struct TV_SAFE_PERIOD_SPEC;
impl crate::RegisterSpec for TV_SAFE_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_safe_period::R](R) reader structure"]
impl crate::Readable for TV_SAFE_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_safe_period::W](W) writer structure"]
impl crate::Writable for TV_SAFE_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tv_safe_period to value 0"]
impl crate::Resettable for TV_SAFE_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
