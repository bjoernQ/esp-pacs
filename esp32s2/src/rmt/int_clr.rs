#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_TX_END_INT_CLR` writer - Set this bit to clear the CH0_TX_END_INT interrupt."]
pub struct CH0_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CH0_RX_END_INT_CLR` writer - Set this bit to clear the CH0_RX_END_INT interrupt."]
pub struct CH0_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_RX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH0_ERR_INT_CLR` writer - Set this bit to clear the CH0_ERR_INT interrupt."]
pub struct CH0_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ERR_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH1_TX_END_INT_CLR` writer - Set this bit to clear the CH1_TX_END_INT interrupt."]
pub struct CH1_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH1_RX_END_INT_CLR` writer - Set this bit to clear the CH1_RX_END_INT interrupt."]
pub struct CH1_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_RX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH1_ERR_INT_CLR` writer - Set this bit to clear the CH1_ERR_INT interrupt."]
pub struct CH1_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ERR_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH2_TX_END_INT_CLR` writer - Set this bit to clear the CH2_TX_END_INT interrupt."]
pub struct CH2_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CH2_RX_END_INT_CLR` writer - Set this bit to clear the CH2_RX_END_INT interrupt."]
pub struct CH2_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CH2_ERR_INT_CLR` writer - Set this bit to clear the CH2_ERR_INT interrupt."]
pub struct CH2_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ERR_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH3_TX_END_INT_CLR` writer - Set this bit to clear the CH3_TX_END_INT interrupt."]
pub struct CH3_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH3_RX_END_INT_CLR` writer - Set this bit to clear the CH3_RX_END_INT interrupt."]
pub struct CH3_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RX_END_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CH3_ERR_INT_CLR` writer - Set this bit to clear the CH3_ERR_INT interrupt."]
pub struct CH3_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ERR_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_CLR` writer - Set this bit to clear the CH0_TX_THR_EVENT_INT interrupt."]
pub struct CH0_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_THR_EVENT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_CLR` writer - Set this bit to clear the CH1_TX_THR_EVENT_INT interrupt."]
pub struct CH1_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_THR_EVENT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CH2_TX_THR_EVENT_INT_CLR` writer - Set this bit to clear the CH2_TX_THR_EVENT_INT interrupt."]
pub struct CH2_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_THR_EVENT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CH3_TX_THR_EVENT_INT_CLR` writer - Set this bit to clear the CH3_TX_THR_EVENT_INT interrupt."]
pub struct CH3_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_THR_EVENT_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_CLR` writer - Set this bit to clear the CH0_TX_LOOP_INT interrupt."]
pub struct CH0_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_LOOP_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_CLR` writer - Set this bit to clear the CH1_TX_LOOP_INT interrupt."]
pub struct CH1_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_LOOP_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CH2_TX_LOOP_INT_CLR` writer - Set this bit to clear the CH2_TX_LOOP_INT interrupt."]
pub struct CH2_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_TX_LOOP_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH3_TX_LOOP_INT_CLR` writer - Set this bit to clear the CH3_TX_LOOP_INT interrupt."]
pub struct CH3_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_TX_LOOP_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the CH0_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_end_int_clr(&mut self) -> CH0_TX_END_INT_CLR_W {
        CH0_TX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to clear the CH0_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_rx_end_int_clr(&mut self) -> CH0_RX_END_INT_CLR_W {
        CH0_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to clear the CH0_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch0_err_int_clr(&mut self) -> CH0_ERR_INT_CLR_W {
        CH0_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to clear the CH1_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_end_int_clr(&mut self) -> CH1_TX_END_INT_CLR_W {
        CH1_TX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to clear the CH1_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_rx_end_int_clr(&mut self) -> CH1_RX_END_INT_CLR_W {
        CH1_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to clear the CH1_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch1_err_int_clr(&mut self) -> CH1_ERR_INT_CLR_W {
        CH1_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to clear the CH2_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_end_int_clr(&mut self) -> CH2_TX_END_INT_CLR_W {
        CH2_TX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7 - Set this bit to clear the CH2_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_rx_end_int_clr(&mut self) -> CH2_RX_END_INT_CLR_W {
        CH2_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8 - Set this bit to clear the CH2_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch2_err_int_clr(&mut self) -> CH2_ERR_INT_CLR_W {
        CH2_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9 - Set this bit to clear the CH3_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_end_int_clr(&mut self) -> CH3_TX_END_INT_CLR_W {
        CH3_TX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10 - Set this bit to clear the CH3_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_rx_end_int_clr(&mut self) -> CH3_RX_END_INT_CLR_W {
        CH3_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11 - Set this bit to clear the CH3_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch3_err_int_clr(&mut self) -> CH3_ERR_INT_CLR_W {
        CH3_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12 - Set this bit to clear the CH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_clr(&mut self) -> CH0_TX_THR_EVENT_INT_CLR_W {
        CH0_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13 - Set this bit to clear the CH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_clr(&mut self) -> CH1_TX_THR_EVENT_INT_CLR_W {
        CH1_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14 - Set this bit to clear the CH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_clr(&mut self) -> CH2_TX_THR_EVENT_INT_CLR_W {
        CH2_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15 - Set this bit to clear the CH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_clr(&mut self) -> CH3_TX_THR_EVENT_INT_CLR_W {
        CH3_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Set this bit to clear the CH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_clr(&mut self) -> CH0_TX_LOOP_INT_CLR_W {
        CH0_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to clear the CH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_clr(&mut self) -> CH1_TX_LOOP_INT_CLR_W {
        CH1_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18 - Set this bit to clear the CH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_clr(&mut self) -> CH2_TX_LOOP_INT_CLR_W {
        CH2_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to clear the CH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_clr(&mut self) -> CH3_TX_LOOP_INT_CLR_W {
        CH3_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`]
(crate::generic::Reg::write_with_zero), [`reset`]
(crate::generic::Reg::reset), [`write`]
(crate::generic::Reg::write). See [API]
(https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr]
(index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W]
(W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}