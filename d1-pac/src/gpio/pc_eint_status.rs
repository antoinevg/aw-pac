#[doc = "Register `pc_eint_status` reader"]
pub struct R(crate::R<PC_EINT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_EINT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_EINT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_EINT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pc_eint_status` writer"]
pub struct W(crate::W<PC_EINT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_EINT_STATUS_SPEC>;
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
impl From<crate::W<PC_EINT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_EINT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `eint_status[0-7]` reader - External INT Pending Bit"]
pub type EINT_STATUS_R = crate::BitReader<EINT_STATUS_A>;
#[doc = "External INT Pending Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT_STATUS_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<EINT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT_STATUS_A {
        match self.bits {
            false => EINT_STATUS_A::NO_PENDING,
            true => EINT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == EINT_STATUS_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EINT_STATUS_A::PENDING
    }
}
#[doc = "Field `eint_status[0-7]` writer - External INT Pending Bit"]
pub type EINT_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PC_EINT_STATUS_SPEC, EINT_STATUS_A, O>;
impl<'a, const O: u8> EINT_STATUS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(EINT_STATUS_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(EINT_STATUS_A::PENDING)
    }
}
impl R {
    #[doc = "External INT Pending Bit"]
    #[inline(always)]
    pub unsafe fn eint_status(&self, n: u8) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint0_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint1_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint2_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint3_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint4_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint5_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint6_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint7_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "External INT Pending Bit"]
    #[inline(always)]
    pub unsafe fn eint_status<const O: u8>(&mut self) -> EINT_STATUS_W<O> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint0_status(&mut self) -> EINT_STATUS_W<0> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint1_status(&mut self) -> EINT_STATUS_W<1> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint2_status(&mut self) -> EINT_STATUS_W<2> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint3_status(&mut self) -> EINT_STATUS_W<3> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint4_status(&mut self) -> EINT_STATUS_W<4> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint5_status(&mut self) -> EINT_STATUS_W<5> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint6_status(&mut self) -> EINT_STATUS_W<6> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Bit 7 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint7_status(&mut self) -> EINT_STATUS_W<7> {
        EINT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC External Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_eint_status](index.html) module"]
pub struct PC_EINT_STATUS_SPEC;
impl crate::RegisterSpec for PC_EINT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_eint_status::R](R) reader structure"]
impl crate::Readable for PC_EINT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_eint_status::W](W) writer structure"]
impl crate::Writable for PC_EINT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pc_eint_status to value 0"]
impl crate::Resettable for PC_EINT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
