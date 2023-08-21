#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `RXFIFO_FULL_THRHD` reader - An UART_RXFIFO_FULL_INT interrupt is generated when the receiver receives more data than this register’s value."]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - An UART_RXFIFO_FULL_INT interrupt is generated when the receiver receives more data than this register’s value."]
pub type RXFIFO_FULL_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - An UART_TXFIFO_EMPTY_INT interrupt is generated when the number of data bytes in TX FIFO is less than this register's value."]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - An UART_TXFIFO_EMPTY_INT interrupt is generated when the number of data bytes in TX FIFO is less than this register's value."]
pub type TXFIFO_EMPTY_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `RX_TOUT_FLOW_DIS` reader - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RX_TOUT_FLOW_DIS_R = crate::BitReader;
#[doc = "Field `RX_TOUT_FLOW_DIS` writer - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub type RX_TOUT_FLOW_DIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for UART receiver. 1: Choose software flow control with configuring sw_rts signal. 0: Disable software flow control."]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for UART receiver. 1: Choose software flow control with configuring sw_rts signal. 0: Disable software flow control."]
pub type RX_FLOW_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_TOUT_EN` reader - This is the enable bit for UART receiver's timeout function."]
pub type RX_TOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TOUT_EN` writer - This is the enable bit for UART receiver's timeout function."]
pub type RX_TOUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:8 - An UART_RXFIFO_FULL_INT interrupt is generated when the receiver receives more data than this register’s value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - An UART_TXFIFO_EMPTY_INT interrupt is generated when the number of data bytes in TX FIFO is less than this register's value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&self) -> RX_TOUT_FLOW_DIS_R {
        RX_TOUT_FLOW_DIS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This is the flow enable bit for UART receiver. 1: Choose software flow control with configuring sw_rts signal. 0: Disable software flow control."]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This is the enable bit for UART receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "rxfifo_full_thrhd",
                &format_args!("{}", self.rxfifo_full_thrhd().bits()),
            )
            .field(
                "txfifo_empty_thrhd",
                &format_args!("{}", self.txfifo_empty_thrhd().bits()),
            )
            .field(
                "rx_tout_flow_dis",
                &format_args!("{}", self.rx_tout_flow_dis().bit()),
            )
            .field("rx_flow_en", &format_args!("{}", self.rx_flow_en().bit()))
            .field("rx_tout_en", &format_args!("{}", self.rx_tout_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - An UART_RXFIFO_FULL_INT interrupt is generated when the receiver receives more data than this register’s value."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<CONF1_SPEC, 0> {
        RXFIFO_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 9:17 - An UART_TXFIFO_EMPTY_INT interrupt is generated when the number of data bytes in TX FIFO is less than this register's value."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<CONF1_SPEC, 9> {
        TXFIFO_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_flow_dis(&mut self) -> RX_TOUT_FLOW_DIS_W<CONF1_SPEC, 29> {
        RX_TOUT_FLOW_DIS_W::new(self)
    }
    #[doc = "Bit 30 - This is the flow enable bit for UART receiver. 1: Choose software flow control with configuring sw_rts signal. 0: Disable software flow control."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<CONF1_SPEC, 30> {
        RX_FLOW_EN_W::new(self)
    }
    #[doc = "Bit 31 - This is the enable bit for UART receiver's timeout function."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W<CONF1_SPEC, 31> {
        RX_TOUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0xc060"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc060;
}
