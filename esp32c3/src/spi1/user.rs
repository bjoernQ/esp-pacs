#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK_OUT_EDGE` reader - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
pub type CK_OUT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CK_OUT_EDGE` writer - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
pub type CK_OUT_EDGE_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 9>;
#[doc = "Field `FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals"]
pub type FWRITE_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals"]
pub type FWRITE_DUAL_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 12>;
#[doc = "Field `FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals"]
pub type FWRITE_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals"]
pub type FWRITE_QUAD_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 13>;
#[doc = "Field `FWRITE_DIO` reader - In the write operations address phase and read-data phase apply 2 signals."]
pub type FWRITE_DIO_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_DIO` writer - In the write operations address phase and read-data phase apply 2 signals."]
pub type FWRITE_DIO_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 14>;
#[doc = "Field `FWRITE_QIO` reader - In the write operations address phase and read-data phase apply 4 signals."]
pub type FWRITE_QIO_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_QIO` writer - In the write operations address phase and read-data phase apply 4 signals."]
pub type FWRITE_QIO_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 15>;
#[doc = "Field `USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_R = crate::BitReader<bool>;
#[doc = "Field `USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 24>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_R = crate::BitReader<bool>;
#[doc = "Field `USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 25>;
#[doc = "Field `USR_DUMMY_IDLE` reader - SPI clock is disable in dummy phase when the bit is enable."]
pub type USR_DUMMY_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY_IDLE` writer - SPI clock is disable in dummy phase when the bit is enable."]
pub type USR_DUMMY_IDLE_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 26>;
#[doc = "Field `USR_MOSI` reader - This bit enable the write-data phase of an operation."]
pub type USR_MOSI_R = crate::BitReader<bool>;
#[doc = "Field `USR_MOSI` writer - This bit enable the write-data phase of an operation."]
pub type USR_MOSI_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 27>;
#[doc = "Field `USR_MISO` reader - This bit enable the read-data phase of an operation."]
pub type USR_MISO_R = crate::BitReader<bool>;
#[doc = "Field `USR_MISO` writer - This bit enable the read-data phase of an operation."]
pub type USR_MISO_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 28>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type USR_DUMMY_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type USR_DUMMY_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 29>;
#[doc = "Field `USR_ADDR` reader - This bit enable the address phase of an operation."]
pub type USR_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `USR_ADDR` writer - This bit enable the address phase of an operation."]
pub type USR_ADDR_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 30>;
#[doc = "Field `USR_COMMAND` reader - This bit enable the command phase of an operation."]
pub type USR_COMMAND_R = crate::BitReader<bool>;
#[doc = "Field `USR_COMMAND` writer - This bit enable the command phase of an operation."]
pub type USR_COMMAND_W<'a> = crate::BitWriter<'a, u32, USER_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SPI clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W {
        CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W {
        FWRITE_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W {
        FWRITE_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W {
        FWRITE_DIO_W::new(self)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W {
        FWRITE_QIO_W::new(self)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W {
        USR_MISO_HIGHPART_W::new(self)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W {
        USR_MOSI_HIGHPART_W::new(self)
    }
    #[doc = "Bit 26 - SPI clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W {
        USR_DUMMY_IDLE_W::new(self)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W {
        USR_MOSI_W::new(self)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W {
        USR_MISO_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W {
        USR_DUMMY_W::new(self)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W {
        USR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W {
        USR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER to value 0x8000_0000"]
impl crate::Resettable for USER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
