#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_ADDR` reader - This register stores the current receive descriptor's address."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IN_DSCR_STATE` reader - Reserved."]
pub type IN_DSCR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_STATE` reader - Reserved."]
pub type IN_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INFIFO_CNT_DEBUG` reader - This register stores the number of data bytes in RX FIFO."]
pub type INFIFO_CNT_DEBUG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECODE_STATE` reader - UHCI decoder status."]
pub type DECODE_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:17 - This register stores the current receive descriptor's address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - Reserved."]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Reserved."]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:27 - This register stores the number of data bytes in RX FIFO."]
    #[inline(always)]
    pub fn infifo_cnt_debug(&self) -> INFIFO_CNT_DEBUG_R {
        INFIFO_CNT_DEBUG_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30 - UHCI decoder status."]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "UHCI decoder status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}