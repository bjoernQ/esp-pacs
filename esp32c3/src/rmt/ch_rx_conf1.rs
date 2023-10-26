#[doc = "Register `CH%s_RX_CONF1` reader"]
pub type R = crate::R<CH_RX_CONF1_SPEC>;
#[doc = "Register `CH%s_RX_CONF1` writer"]
pub type W = crate::W<CH_RX_CONF1_SPEC>;
#[doc = "Field `RX_EN` reader - reg_rx_en_ch2."]
pub type RX_EN_R = crate::BitReader;
#[doc = "Field `RX_EN` writer - reg_rx_en_ch2."]
pub type RX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_WR_RST` writer - reg_mem_wr_rst_ch2."]
pub type MEM_WR_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_MEM_RST` writer - reg_apb_mem_rst_ch2."]
pub type APB_MEM_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_OWNER` reader - reg_mem_owner_ch2."]
pub type MEM_OWNER_R = crate::BitReader;
#[doc = "Field `MEM_OWNER` writer - reg_mem_owner_ch2."]
pub type MEM_OWNER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FILTER_EN` reader - reg_rx_filter_en_ch2."]
pub type RX_FILTER_EN_R = crate::BitReader;
#[doc = "Field `RX_FILTER_EN` writer - reg_rx_filter_en_ch2."]
pub type RX_FILTER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FILTER_THRES` reader - reg_rx_filter_thres_ch2."]
pub type RX_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `RX_FILTER_THRES` writer - reg_rx_filter_thres_ch2."]
pub type RX_FILTER_THRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MEM_RX_WRAP_EN` reader - reg_mem_rx_wrap_en_ch2."]
pub type MEM_RX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_RX_WRAP_EN` writer - reg_mem_rx_wrap_en_ch2."]
pub type MEM_RX_WRAP_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AFIFO_RST` writer - reg_afifo_rst_ch2."]
pub type AFIFO_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CONF_UPDATE` writer - reg_conf_update_ch2."]
pub type CONF_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - reg_rx_en_ch2."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch2."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch2."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch2."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch2."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&self) -> MEM_RX_WRAP_EN_R {
        MEM_RX_WRAP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF1")
            .field("rx_en", &format_args!("{}", self.rx_en().bit()))
            .field("mem_owner", &format_args!("{}", self.mem_owner().bit()))
            .field(
                "rx_filter_en",
                &format_args!("{}", self.rx_filter_en().bit()),
            )
            .field(
                "rx_filter_thres",
                &format_args!("{}", self.rx_filter_thres().bits()),
            )
            .field(
                "mem_rx_wrap_en",
                &format_args!("{}", self.mem_rx_wrap_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_RX_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rx_en_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<CH_RX_CONF1_SPEC, 0> {
        RX_EN_W::new(self)
    }
    #[doc = "Bit 1 - reg_mem_wr_rst_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W<CH_RX_CONF1_SPEC, 1> {
        MEM_WR_RST_W::new(self)
    }
    #[doc = "Bit 2 - reg_apb_mem_rst_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<CH_RX_CONF1_SPEC, 2> {
        APB_MEM_RST_W::new(self)
    }
    #[doc = "Bit 3 - reg_mem_owner_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W<CH_RX_CONF1_SPEC, 3> {
        MEM_OWNER_W::new(self)
    }
    #[doc = "Bit 4 - reg_rx_filter_en_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W<CH_RX_CONF1_SPEC, 4> {
        RX_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 5:12 - reg_rx_filter_thres_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W<CH_RX_CONF1_SPEC, 5> {
        RX_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 13 - reg_mem_rx_wrap_en_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn mem_rx_wrap_en(&mut self) -> MEM_RX_WRAP_EN_W<CH_RX_CONF1_SPEC, 13> {
        MEM_RX_WRAP_EN_W::new(self)
    }
    #[doc = "Bit 14 - reg_afifo_rst_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<CH_RX_CONF1_SPEC, 14> {
        AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 15 - reg_conf_update_ch2."]
    #[inline(always)]
    #[must_use]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<CH_RX_CONF1_SPEC, 15> {
        CONF_UPDATE_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT_CH2CONF1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_rx_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_rx_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CONF1_SPEC;
impl crate::RegisterSpec for CH_RX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_conf1::R`](R) reader structure"]
impl crate::Readable for CH_RX_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_conf1::W`](W) writer structure"]
impl crate::Writable for CH_RX_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_RX_CONF1 to value 0x01e8"]
impl crate::Resettable for CH_RX_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01e8;
}
