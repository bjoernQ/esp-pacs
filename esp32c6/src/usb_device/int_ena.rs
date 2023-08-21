#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `JTAG_IN_FLUSH_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_ENA_R = crate::BitReader;
#[doc = "Field `JTAG_IN_FLUSH_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
pub type JTAG_IN_FLUSH_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOF_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SOF_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
pub type SOF_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SERIAL_OUT_RECV_PKT_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
pub type SERIAL_OUT_RECV_PKT_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERIAL_IN_EMPTY_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `SERIAL_IN_EMPTY_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
pub type SERIAL_IN_EMPTY_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PID_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `PID_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
pub type PID_ERR_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC5_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CRC5_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
pub type CRC5_ERR_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRC16_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CRC16_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
pub type CRC16_ERR_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUFF_ERR_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `STUFF_ERR_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
pub type STUFF_ERR_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_TOKEN_REC_IN_EP1_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
pub type IN_TOKEN_REC_IN_EP1_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_BUS_RESET_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_ENA_R = crate::BitReader;
#[doc = "Field `USB_BUS_RESET_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
pub type USB_BUS_RESET_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EP1_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EP2_ZERO_PAYLOAD_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
pub type OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTS_CHG_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type RTS_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RTS_CHG_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
pub type RTS_CHG_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTR_CHG_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type DTR_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `DTR_CHG_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
pub type DTR_CHG_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type GET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `GET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
pub type GET_LINE_CODE_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SET_LINE_CODE_INT_ENA` reader - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type SET_LINE_CODE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SET_LINE_CODE_INT_ENA` writer - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
pub type SET_LINE_CODE_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    pub fn jtag_in_flush_int_ena(&self) -> JTAG_IN_FLUSH_INT_ENA_R {
        JTAG_IN_FLUSH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    pub fn sof_int_ena(&self) -> SOF_INT_ENA_R {
        SOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    pub fn serial_out_recv_pkt_int_ena(&self) -> SERIAL_OUT_RECV_PKT_INT_ENA_R {
        SERIAL_OUT_RECV_PKT_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    pub fn serial_in_empty_int_ena(&self) -> SERIAL_IN_EMPTY_INT_ENA_R {
        SERIAL_IN_EMPTY_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    pub fn pid_err_int_ena(&self) -> PID_ERR_INT_ENA_R {
        PID_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc5_err_int_ena(&self) -> CRC5_ERR_INT_ENA_R {
        CRC5_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    pub fn crc16_err_int_ena(&self) -> CRC16_ERR_INT_ENA_R {
        CRC16_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn stuff_err_int_ena(&self) -> STUFF_ERR_INT_ENA_R {
        STUFF_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    pub fn in_token_rec_in_ep1_int_ena(&self) -> IN_TOKEN_REC_IN_EP1_INT_ENA_R {
        IN_TOKEN_REC_IN_EP1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    pub fn usb_bus_reset_int_ena(&self) -> USB_BUS_RESET_INT_ENA_R {
        USB_BUS_RESET_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep1_zero_payload_int_ena(&self) -> OUT_EP1_ZERO_PAYLOAD_INT_ENA_R {
        OUT_EP1_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    pub fn out_ep2_zero_payload_int_ena(&self) -> OUT_EP2_ZERO_PAYLOAD_INT_ENA_R {
        OUT_EP2_ZERO_PAYLOAD_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    pub fn rts_chg_int_ena(&self) -> RTS_CHG_INT_ENA_R {
        RTS_CHG_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    pub fn dtr_chg_int_ena(&self) -> DTR_CHG_INT_ENA_R {
        DTR_CHG_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn get_line_code_int_ena(&self) -> GET_LINE_CODE_INT_ENA_R {
        GET_LINE_CODE_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    pub fn set_line_code_int_ena(&self) -> SET_LINE_CODE_INT_ENA_R {
        SET_LINE_CODE_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "jtag_in_flush_int_ena",
                &format_args!("{}", self.jtag_in_flush_int_ena().bit()),
            )
            .field("sof_int_ena", &format_args!("{}", self.sof_int_ena().bit()))
            .field(
                "serial_out_recv_pkt_int_ena",
                &format_args!("{}", self.serial_out_recv_pkt_int_ena().bit()),
            )
            .field(
                "serial_in_empty_int_ena",
                &format_args!("{}", self.serial_in_empty_int_ena().bit()),
            )
            .field(
                "pid_err_int_ena",
                &format_args!("{}", self.pid_err_int_ena().bit()),
            )
            .field(
                "crc5_err_int_ena",
                &format_args!("{}", self.crc5_err_int_ena().bit()),
            )
            .field(
                "crc16_err_int_ena",
                &format_args!("{}", self.crc16_err_int_ena().bit()),
            )
            .field(
                "stuff_err_int_ena",
                &format_args!("{}", self.stuff_err_int_ena().bit()),
            )
            .field(
                "in_token_rec_in_ep1_int_ena",
                &format_args!("{}", self.in_token_rec_in_ep1_int_ena().bit()),
            )
            .field(
                "usb_bus_reset_int_ena",
                &format_args!("{}", self.usb_bus_reset_int_ena().bit()),
            )
            .field(
                "out_ep1_zero_payload_int_ena",
                &format_args!("{}", self.out_ep1_zero_payload_int_ena().bit()),
            )
            .field(
                "out_ep2_zero_payload_int_ena",
                &format_args!("{}", self.out_ep2_zero_payload_int_ena().bit()),
            )
            .field(
                "rts_chg_int_ena",
                &format_args!("{}", self.rts_chg_int_ena().bit()),
            )
            .field(
                "dtr_chg_int_ena",
                &format_args!("{}", self.dtr_chg_int_ena().bit()),
            )
            .field(
                "get_line_code_int_ena",
                &format_args!("{}", self.get_line_code_int_ena().bit()),
            )
            .field(
                "set_line_code_int_ena",
                &format_args!("{}", self.set_line_code_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the USB_DEVICE_JTAG_IN_FLUSH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_in_flush_int_ena(&mut self) -> JTAG_IN_FLUSH_INT_ENA_W<INT_ENA_SPEC, 0> {
        JTAG_IN_FLUSH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the USB_DEVICE_SOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sof_int_ena(&mut self) -> SOF_INT_ENA_W<INT_ENA_SPEC, 1> {
        SOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the USB_DEVICE_SERIAL_OUT_RECV_PKT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn serial_out_recv_pkt_int_ena(
        &mut self,
    ) -> SERIAL_OUT_RECV_PKT_INT_ENA_W<INT_ENA_SPEC, 2> {
        SERIAL_OUT_RECV_PKT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the USB_DEVICE_SERIAL_IN_EMPTY_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn serial_in_empty_int_ena(&mut self) -> SERIAL_IN_EMPTY_INT_ENA_W<INT_ENA_SPEC, 3> {
        SERIAL_IN_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the USB_DEVICE_PID_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pid_err_int_ena(&mut self) -> PID_ERR_INT_ENA_W<INT_ENA_SPEC, 4> {
        PID_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the USB_DEVICE_CRC5_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc5_err_int_ena(&mut self) -> CRC5_ERR_INT_ENA_W<INT_ENA_SPEC, 5> {
        CRC5_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the USB_DEVICE_CRC16_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn crc16_err_int_ena(&mut self) -> CRC16_ERR_INT_ENA_W<INT_ENA_SPEC, 6> {
        CRC16_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the USB_DEVICE_STUFF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stuff_err_int_ena(&mut self) -> STUFF_ERR_INT_ENA_W<INT_ENA_SPEC, 7> {
        STUFF_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the USB_DEVICE_IN_TOKEN_REC_IN_EP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_token_rec_in_ep1_int_ena(
        &mut self,
    ) -> IN_TOKEN_REC_IN_EP1_INT_ENA_W<INT_ENA_SPEC, 8> {
        IN_TOKEN_REC_IN_EP1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the USB_DEVICE_USB_BUS_RESET_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn usb_bus_reset_int_ena(&mut self) -> USB_BUS_RESET_INT_ENA_W<INT_ENA_SPEC, 9> {
        USB_BUS_RESET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the USB_DEVICE_OUT_EP1_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep1_zero_payload_int_ena(
        &mut self,
    ) -> OUT_EP1_ZERO_PAYLOAD_INT_ENA_W<INT_ENA_SPEC, 10> {
        OUT_EP1_ZERO_PAYLOAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the USB_DEVICE_OUT_EP2_ZERO_PAYLOAD_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep2_zero_payload_int_ena(
        &mut self,
    ) -> OUT_EP2_ZERO_PAYLOAD_INT_ENA_W<INT_ENA_SPEC, 11> {
        OUT_EP2_ZERO_PAYLOAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the USB_DEVICE_RTS_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rts_chg_int_ena(&mut self) -> RTS_CHG_INT_ENA_W<INT_ENA_SPEC, 12> {
        RTS_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for the USB_DEVICE_DTR_CHG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dtr_chg_int_ena(&mut self) -> DTR_CHG_INT_ENA_W<INT_ENA_SPEC, 13> {
        DTR_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for the USB_DEVICE_GET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn get_line_code_int_ena(&mut self) -> GET_LINE_CODE_INT_ENA_W<INT_ENA_SPEC, 14> {
        GET_LINE_CODE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for the USB_DEVICE_SET_LINE_CODE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn set_line_code_int_ena(&mut self) -> SET_LINE_CODE_INT_ENA_W<INT_ENA_SPEC, 15> {
        SET_LINE_CODE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
